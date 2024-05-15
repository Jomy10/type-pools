use std::{
    any::{Any, TypeId},
    collections::{hash_map::RandomState, HashMap},
};

/// A collection of pools or arrays that contain values of a specific type
pub struct TypePools<H = RandomState> {
    pools: HashMap<TypeId, Box<dyn TypePoolTrait>, H>,
}

impl TypePools {
    pub fn new() -> Self {
        TypePools { pools: HashMap::new() }
    }

    /// Get a reference to a type pool
    pub fn type_pool<T: 'static>(&self) -> Option<&TypePool<T>> {
        self.pools.get(&TypeId::of::<T>())
            .map(|pool| {
                unsafe { TypePool::<T>::cast_unchecked(pool.as_ref()) } // safety: we know the type is correct
            })
    }

    /// Get a mutable reference to a type pool
    pub fn type_pool_mut<T: 'static>(&mut self) -> Option<&mut TypePool<T>> {
        self.pools.get_mut(&TypeId::of::<T>())
            .map(|pool| {
                unsafe { TypePool::<T>::cast_mut_unchecked(pool.as_mut()) } // safety: we know the type is correct
            })
    }

    /// Add a value to the pools. If the type pool doesn't exst yet, it will be created
    pub fn push<T: 'static>(&mut self, value: T) {
        let pools = self.pools.get_mut(&TypeId::of::<T>());
        if let Some(pools) = pools {
            unsafe { TypePool::<T>::cast_mut_unchecked(pools.as_mut()) }
                .values.push(value);
        } else {
            self.pools.insert(TypeId::of::<T>(), Box::new(TypePool::<T>::new()));
            unsafe { TypePool::<T>::cast_mut_unchecked(self.pools.get_mut(&TypeId::of::<T>()).unwrap_unchecked().as_mut()) } // safety: I litterrally just created it
                .values.push(value)
        }
    }

    /// Returns the popped item or `None` if the value doesn't exist
    pub fn pop<T: 'static>(&mut self) -> Option<T> {
        self.type_pool_mut()
            .and_then(|p| p.values.pop())
    }

    /// Remove the value at the index in the type pool specified by `T`
    pub fn remove<T: 'static>(&mut self, idx: usize) -> Option<T> {
        self.type_pool_mut()
            .and_then(|p| p.values.remove(idx))
    }

    /// Gets a value from a TypePool
    ///
    /// # Parameters
    /// - idx: this is the index in the specific type `T` array
    pub fn get<T: 'static>(&self, idx: usize) -> Option<&T> {
        self.type_pool()
            .and_then(|p| p.values.get(idx))
    }

    /// Get a mutable reference to a value in a TypePool
    pub fn get_mut<T: 'static>(&mut self, idx: usize) -> Option<&mut T> {
        self.type_pool_mut()
            .and_then(|p| p.values.get_mut(idx))
    }

    pub fn len<T: 'static>(&self) -> Option<usize> {
        self.type_pool()
            .map(|f: &TypePool<T>| f.values.len())
    }

    /// The amount of types stored in pools
    pub fn types_count(&self) -> usize {
        self.pools.keys().len()
    }

    /// The types stored in pools
    pub fn types(&self) -> Vec<&TypeId> {
        self.pools.keys().collect()
    }

    /// Remove all entries for a type
    pub fn remove_type<T: 'static>(&mut self) {
        self.pools.remove(&TypeId::of::<T>());
    }

    /// Remove all types which do not contain any values
    pub fn remove_empty(&mut self) {
        let to_remove = self.pools.keys()
            // safety: the keys exist
            .filter(|key| unsafe{ self.pools.get(key).unwrap_unchecked() }.is_empty())
            .map(|key| *key)
            .collect::<Vec<TypeId>>();

        for id in to_remove {
            self.pools.remove(&id);
        }
    }

    /// Shrink the array containing all the pools to fit
    pub fn shrink_to_fit(&mut self) {
        self.pools.shrink_to_fit()
    }

    

    // /// Returns `None` when the type does not exist in the pools
    // pub fn reserve<T: 'static>(&mut self, additional: usize) -> Option<()> {
    //     self.type_pool_mut()
    //         .map(|p: &mut TypePool<T>| p.values.reserve(additional))
    // }
    //
    // pub fn reserve_exact<T: 'static>(&mut self, additional: usize) -> Option<()> {
    //     self.type_pool_mut()
    //         .map(|p: &mut TypePool<T>| p.values.reserve_exact(additional))
    // }
    //
    // pub fn try_reserve<T: 'static>(&mut self, additional: usize) -> Option<Result<(), TryReserveError>> {
    //     self.type_pool_mut()
    //         .map(|p: &mut TypePool<T>| p.values.try_reserve(additional))
    // }
    //
    // pub fn try_reserve_exact<T: 'static>(&mut self, additional: usize) -> Option<Result<(), TryReserveError>> {
    //     self.type_pool_mut()
    //         .map(|p: &mut TypePool<T>| p.values.try_reserve_exact(additional))
    // }
    //
    // pub fn shrink_to_fit<T: 'static>(&mut self) -> Option<()> {
    //     self.type_pool_mut()
    //         .map(|p: &mut TypePool<T>| p.values.shrink_to_fit())
    // }

    // TODO: implement other vec methods
}

trait TypePoolTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn is_empty(&self) -> bool;
}

pub struct TypePool<T> {
    pub values: Vec<T>,
}

impl<T: 'static> TypePool<T> {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    // fn cast(pool: &dyn TypePoolTrait) -> &Self {
    //     pool.as_any()
    //         .downcast_ref::<TypePool<T>>()
    //         .unwrap()
    // }
    //
    // fn cast_mut(pool: &mut dyn TypePoolTrait) -> &mut Self {
    //     pool.as_any_mut()
    //         .downcast_mut::<TypePool<T>>()
    //         .unwrap()
    // }

    unsafe fn cast_unchecked(pool: &dyn TypePoolTrait) -> &Self {
        pool.as_any()
            .downcast_ref::<TypePool<T>>()
            .unwrap_unchecked()
    }

    unsafe fn cast_mut_unchecked(pool: &mut dyn TypePoolTrait) -> &mut Self {
        pool.as_any_mut()
            .downcast_mut::<TypePool<T>>()
            .unwrap_unchecked()
    }
}

impl<T: 'static> TypePoolTrait for TypePool<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::TypePools;

    #[test]
    fn test_add() {
        let mut pools = TypePools::new();
        pools.push(1 as u32);
        pools.push(2 as u32);
        pools.push("Hello");
        pools.push("World");

        assert_eq!(*pools.get::<u32>(0).unwrap(), 1);
        assert_eq!(*pools.get::<u32>(1).unwrap(), 2);
        assert_eq!(*pools.get::<&str>(0).unwrap(), "Hello");
        assert_eq!(*pools.get::<&str>(1).unwrap(), "World");
    }

    #[test]
    fn test_example() {
        // Create a Type Pools structure
        let mut pools = TypePools::new();

        // Adding values
        pools.push(1 as u32);
        pools.push(2 as u32);
        pools.push("Hello world");

        // Query values
        let int_pool = pools.type_pool::<u32>().unwrap();
        let int_value: u32 = int_pool.values[0];
        let string_value: &str = pools.get::<&str>(0).unwrap();

        assert_eq!(int_value, 1);
        assert_eq!(string_value, "Hello world");
    }
}

