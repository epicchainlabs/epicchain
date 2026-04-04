//! Compound type operation OpCodes for the Neo Virtual Machine.
//!
//! This module contains all OpCodes related to compound data types,
//! including arrays, structs, maps, and their manipulation.

/// Compound type operation OpCodes.
///
/// These opcodes work with arrays, structs, maps, and other compound data types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompoundOpCode {
    /// Creates a new array with the specified number of elements.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + n elements
    /// ```
    NEWARRAY = 0xC0,

    /// Creates a new array with the specified type and number of elements.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + n elements
    /// ```
    NewarrayT = 0xC1,

    /// Creates a new struct with the specified number of fields.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + n fields
    /// ```
    NEWSTRUCT = 0xC2,

    /// Creates a new map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    NEWMAP = 0xC3,

    /// Appends an element to an array.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    APPEND = 0xC4,

    /// Reverses the elements of an array.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    REVERSE = 0xC5,

    /// Removes an element from an array or map.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    REMOVE = 0xC6,

    /// Checks if a map contains a key or an array contains an index.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    HASKEY = 0xC7,

    /// Returns all keys of a map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    KEYS = 0xC8,

    /// Returns all values of a map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    VALUES = 0xC9,

    /// Packs key-value pairs into a map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + 2n items
    /// ```
    PACKMAP = 0xCA,

    /// Packs values into a struct.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + n items
    /// ```
    PACKSTRUCT = 0xCB,

    /// Packs values into an array.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + n items
    /// ```
    PACK = 0xCC,

    /// Unpacks an array into individual elements.
    ///
    /// # Stack
    /// ```text
    /// Push: n items
    /// Pop: 1 item
    /// ```
    UNPACK = 0xCD,

    /// Gets an element from an array or map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    PICKITEM = 0xCE,

    /// Sets an element in an array or map.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 3 items
    /// ```
    SETITEM = 0xCF,

    /// Returns the size of an array, map, or buffer.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    SIZE = 0xD0,
}

impl CompoundOpCode {
    /// Checks if this operation creates a new compound type.
    pub fn creates_new(&self) -> bool {
        matches!(
            self,
            Self::NEWARRAY
                | Self::NewarrayT
                | Self::NEWSTRUCT
                | Self::NEWMAP
                | Self::PACKMAP
                | Self::PACKSTRUCT
                | Self::PACK
        )
    }

    /// Checks if this operation modifies an existing compound type.
    pub fn modifies_existing(&self) -> bool {
        matches!(
            self,
            Self::APPEND | Self::REVERSE | Self::REMOVE | Self::SETITEM
        )
    }

    /// Checks if this operation queries a compound type.
    pub fn is_query(&self) -> bool {
        matches!(
            self,
            Self::HASKEY | Self::KEYS | Self::VALUES | Self::PICKITEM | Self::SIZE
        )
    }

    /// Checks if this operation works with arrays.
    pub fn works_with_arrays(&self) -> bool {
        matches!(
            self,
            Self::NEWARRAY
                | Self::NewarrayT
                | Self::APPEND
                | Self::REVERSE
                | Self::REMOVE
                | Self::HASKEY
                | Self::PACK
                | Self::UNPACK
                | Self::PICKITEM
                | Self::SETITEM
                | Self::SIZE
        )
    }

    /// Checks if this operation works with maps.
    pub fn works_with_maps(&self) -> bool {
        matches!(
            self,
            Self::NEWMAP
                | Self::REMOVE
                | Self::HASKEY
                | Self::KEYS
                | Self::VALUES
                | Self::PACKMAP
                | Self::PICKITEM
                | Self::SETITEM
                | Self::SIZE
        )
    }

    /// Checks if this operation works with structs.
    pub fn works_with_structs(&self) -> bool {
        matches!(
            self,
            Self::NEWSTRUCT | Self::PACKSTRUCT | Self::PICKITEM | Self::SETITEM
        )
    }
}
