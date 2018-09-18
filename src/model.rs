//! The generic Model type.

pub trait Model<K, I>
where
    K: Copy
{
    fn eval(&self, key: K) -> I;

    fn eval_many(&self, keys: &[K], indices: &mut [I]) {
        for (i, &key) in keys.iter().enumerate() {
            indices[i] = self.eval(key);
        }
    }
}

