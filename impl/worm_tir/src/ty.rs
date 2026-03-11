/// Imports
use id_arena::Id;
use miette::NamedSource;
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};
use worm_ast::atom::Publicity;
use worm_lex::token::Span;
use worm_macros::bug;

/// A field definition within a struct type.
///
/// Holds the name, source location, and uninstantiated type of a single
/// struct field as it appears in the type-checking context.
#[derive(Clone)]
pub struct FieldDef {
    /// Source span covering the full field declaration.
    pub span: Span,
    /// The field's identifier.
    pub name: String,
    /// The field's type before generic instantiation.
    pub ty: Ty,
}

/// A struct type definition as seen by the type system.
///
/// Captures the struct's name, generic parameters, and the ordered list
/// of its fields. Generic parameters are stored as plain name strings;
/// they are resolved to [`Ty::Generic`] indices during type-checking.
#[derive(Clone)]
pub struct StructDef {
    /// Source span covering the full struct declaration.
    pub span: Span,
    /// The struct's identifier.
    pub name: String,
    /// Ordered list of generic parameter names (e.g. `["T", "K"]`).
    pub generics: Vec<String>,
    /// Ordered list of field definitions.
    pub fields: Vec<FieldDef>,
}

/// A single variant within an enum type definition.
///
/// Variants may carry zero or more unnamed payload types (tuple-style).
/// Named-field variants are not currently represented.
#[derive(Clone)]
pub struct VariantDef {
    /// Source span covering the full variant declaration.
    pub span: Span,
    /// The variant's identifier.
    pub name: String,
    /// Ordered payload types before generic instantiation.
    pub fields: Vec<Ty>,
}

/// An enum type definition as seen by the type system.
///
/// Captures the enum's name, generic parameters, and all of its variants.
/// Generic parameters are stored as plain name strings; they are resolved
/// to [`Ty::Generic`] indices during type-checking.
#[derive(Clone)]
pub struct EnumDef {
    /// Source span covering the full enum declaration.
    pub span: Span,
    /// The enum's identifier.
    pub name: String,
    /// Ordered list of generic parameter names (e.g. `["T", "K"]`).
    pub generics: Vec<String>,
    /// Ordered list of variant definitions.
    pub variants: Vec<VariantDef>,
}

/// An algebraic data type (ADT) definition — either a struct or an enum.
///
/// This is the primary unit of user-defined type storage in the type
/// context. Use [`AdtDef::name`], [`AdtDef::as_struct`], and
/// [`AdtDef::as_enum`] for safe access to the inner definition.
#[derive(Clone)]
pub enum AdtDef {
    Struct(StructDef),
    Enum(EnumDef),
}

impl AdtDef {
    /// Returns the name of the ADT regardless of its variant.
    pub fn name(&self) -> String {
        match self {
            AdtDef::Struct(s) => s.name.clone(),
            AdtDef::Enum(e) => e.name.clone(),
        }
    }

    /// Returns a reference to the inner [`StructDef`].
    ///
    /// # Panics
    ///
    /// Triggers a compiler bug if this ADT is an enum, not a struct.
    pub fn as_struct(&self) -> &StructDef {
        match self {
            AdtDef::Enum(_) => bug!("expected struct, got enum by id"),
            AdtDef::Struct(s) => s,
        }
    }

    /// Returns a reference to the inner [`EnumDef`].
    ///
    /// # Panics
    ///
    /// Triggers a compiler bug if this ADT is a struct, not an enum.
    pub fn as_enum(&self) -> &EnumDef {
        match self {
            AdtDef::Struct(_) => bug!("expected struct, got enum by id"),
            AdtDef::Enum(e) => e,
        }
    }
}

/// A function definition as seen by the type system.
///
/// Stores the function's signature in its uninstantiated form — generic
/// parameters are name strings, and parameter/return types may contain
/// [`Ty::Generic`] indices that are resolved at call sites.
pub struct FnDef {
    /// Source span covering the full function declaration.
    pub span: Span,
    /// The function's identifier.
    pub name: String,
    /// Ordered list of generic parameter names (e.g. `["T", "R"]`).
    pub generics: Vec<String>,
    /// Ordered parameter types before generic instantiation.
    pub params: Vec<Ty>,
    /// Return type before generic instantiation.
    pub ret: Ty,
}

/// Discriminates between the two kinds of top-level item definitions.
///
/// Each variant carries the arena ID of the corresponding definition,
/// enabling O(1) lookup in the type context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemDefKind {
    /// An algebraic data type (struct or enum).
    Adt(Id<AdtDef>),
    /// A function or method.
    Fn(Id<FnDef>),
}

/// A top-level item definition with its associated visibility.
///
/// Pairs an [`ItemDefKind`] with the [`Publicity`] declared at the
/// definition site, allowing the resolver to enforce access rules.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemDef {
    /// Visibility of the item (`pub`, `pub(crate)`, private, etc.).
    pub publicity: Publicity,
    /// The concrete kind and arena ID of the definition.
    pub kind: ItemDefKind,
}

/// A compiled module definition.
///
/// Holds the source text (for diagnostic reporting) and a name-indexed
/// map of every top-level item exported or defined by the module.
pub struct ModDef {
    /// The original source file, used to produce [`miette`] diagnostics.
    pub source: Arc<NamedSource<String>>,
    /// All top-level items defined in this module, keyed by their name.
    pub defs: HashMap<String, ItemDef>,
}

/// A type variable used during type inference.
///
/// Each inference variable starts as [`TyVar::Unbound`] and is unified
/// with a concrete type to become [`TyVar::Bound`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TyVar {
    /// Not yet unified with any type.
    Unbound,
    /// Unified with the given type.
    Bound(Ty),
}

/// A list of concrete type arguments supplied to a generic item.
///
/// The length must match the number of generic parameters declared on
/// the corresponding [`AdtDef`] or [`FnDef`].
pub type GenericArgs = Vec<Ty>;

/// The type signature of a first-class function value.
///
/// Unlike [`FnDef`], a `FnSig` does not carry a name or generic
/// parameters — it describes a fully-instantiated callable type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnSig {
    /// Ordered parameter types.
    pub params: Vec<Ty>,
    /// Return type.
    pub ret: Box<Ty>,
}

/// A "meta" type that represents a namespace rather than a value type.
///
/// Meta types appear when an expression refers to a module, an ADT
/// constructor, or an enum variant as a value — i.e. before it is
/// called or instantiated.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetaTy {
    /// A module used as a path prefix (e.g. `std::io`).
    Module(Id<ModDef>),
    /// An ADT used as a constructor (e.g. a struct literal or a
    /// unit-struct value).
    Adt(Id<AdtDef>),
    /// A specific enum variant constructor (e.g. `Option::Some`).
    Variant(Id<AdtDef>, String),
}

/// The central type representation used throughout type-checking and the
/// Typed Intermediate Representation (TIR).
///
/// Every node in the TIR is annotated with a `Ty`. The variants cover
/// primitive types, user-defined ADTs, callable types, generics,
/// inference variables, meta-level namespace types, and an error
/// sentinel for recovery after a type error.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    /// The primitive signed integer type (`int`).
    Int,
    /// The primitive floating-point type (`float`).
    Float,
    /// The primitive string slice type (`str`).
    String,
    /// The primitive boolean type (`bool`).
    Bool,
    /// The unit type `()` — returned by expressions with no value.
    Unit,
    /// An instantiated ADT type, e.g. `List<Int>`.
    Adt(Id<AdtDef>, GenericArgs),
    /// The type of a direct reference to a named function definition,
    /// with generic args instantiated.
    FnDef(Id<FnDef>, GenericArgs),
    /// The type of a first-class function value whose concrete identity
    /// is not statically known, e.g. a closure or function pointer.
    FnSig(FnSig),
    /// A reference to the *n*-th generic parameter of the enclosing
    /// definition (zero-indexed), e.g. `T` or `K`.
    Generic(usize),
    /// A fresh inference variable created during constraint generation.
    /// Resolved to a concrete type by the unification solver.
    Var(Id<TyVar>),
    /// A meta/namespace type — not a value type. See [`MetaTy`].
    Meta(MetaTy),
    /// A sentinel placed wherever a type could not be determined due to
    /// a prior error. Prevents cascading diagnostics.
    Error,
}
