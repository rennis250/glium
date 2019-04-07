use uniforms::{Uniforms, UniformValue};

/// Stores uniforms.
#[derive(Clone)]
pub struct UniformsStorage<'n> {
    map: Vec<(String, UniformValue<'n>)>
}

impl<'n> UniformsStorage<'n> {
    /// Builds a new storage with a value.
    #[inline]
    pub fn new(name: &'n str, value: UniformValue<'n>) -> UniformsStorage<'n> {
        UniformsStorage {
        	map: vec![(name.to_string(), value)],
        }
    }
    
    /// Builds a new storage with a Vec.
    #[inline]
    pub fn new_from_vec(values: &Vec<(String, UniformValue<'n>)>) -> UniformsStorage<'n> {
        UniformsStorage {
        	map: values.clone(),
        }
    }
}

impl<'n> UniformsStorage<'n> {
    /// Adds a value to the storage.
    #[inline]
    pub fn add(&mut self, name: &'n str, value: UniformValue<'n>) -> Self
    {
    	let mut found_it = false;
		for kv in self.map.iter_mut() {
			if kv.0 == name {
				*kv = (name.to_string(), value);
				found_it = true;
				break;
			}
		}
		if !found_it {
			self.map.push((name.to_string(), value));
		}
		(*self).clone()
	}
}

impl<'n> Uniforms for UniformsStorage<'n> {
    #[inline]
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut output: F) {
    	for kv in self.map.iter() {
	        output(&kv.0, kv.1);
	    }
    }
}
