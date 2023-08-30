// The following macros help implementing binary operators for polynomials
// Inspired by polynomial crate
#[macro_export(local_inner_macros)]
macro_rules! impl_op_polynomial {
	($op:ident, $method:ident $(,$requirements:ident)*) => {
		impl<T> $op<Polynomial<T>> for &Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero $(+ $requirements<T, Output = T>)*

		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: Polynomial<T>) -> Polynomial<T> { self.$method(other) }
		}

		impl<T> $op<&Polynomial<T>> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero $(+ $requirements<T, Output = T>)*
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: &Polynomial<T>) -> Polynomial<T> { self.$method(other) }
		}

		impl<T> $op<Polynomial<T>> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero $(+ $requirements<T, Output = T>)*
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: Polynomial<T>) -> Polynomial<T> { (&self).$method(other) }
		}
	};
}

// The following macros allow to add polynomials with numbers
// Rust's specialization feature is not stable yet so we have to duplicate the
// code for each primitive type
#[macro_export(local_inner_macros)]
macro_rules! impl_op_some_primitive {
	($op:ident, $method:ident, $t:ty $(,$requirements:ident)*) => {
		impl<T> $op<Polynomial<T>> for $t
		where T: $op<T, Output = T> + Clone + From<$t> + Zero $(+ $requirements<T, Output = T>)*
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: Polynomial<T>) -> Polynomial<T> { other.$method(Polynomial::from(std::vec![self.into()])) }
		}
		impl<T> $op<&Polynomial<T>> for $t
		where T: $op<T, Output = T> + Clone + From<$t> + Zero $(+ $requirements<T, Output = T>)*
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: &Polynomial<T>) -> Polynomial<T> { other.$method(Polynomial::from(std::vec![self.into()])) }
		}
		impl<T> $op<$t> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + From<$t> + Zero $(+ $requirements<T, Output = T>)*
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: $t) -> Polynomial<T> { self.$method(Polynomial::from(std::vec![other.into()])) }
		}
		impl<T> $op<$t> for &Polynomial<T>
		where T: $op<T, Output = T> + Clone + From<$t> + Zero $(+ $requirements<T, Output = T>)*
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: $t) -> Polynomial<T> { self.$method(Polynomial::from(std::vec![other.into()])) }
		}
	};
}

#[macro_export(local_inner_macros)]
macro_rules! impl_op_all_primitive {
	($op:ident, $method:ident $(,$requirements:ident)*) => {
		duplicate::duplicate! {
			[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]
		impl_op_some_primitive!($op, $method, primitive_type $(,$requirements:ident)*);
		}
	};
}

// The next macro implements the assign versions of the operators
#[macro_export(local_inner_macros)]
macro_rules! impl_assign_op {
	($op:ident, $assign_op:ident, $method:ident, $assign_method: ident $(,$requirements:ident)*) => {
		impl<T> $assign_op<Polynomial<T>> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero $(+ $requirements<T, Output = T>)*
		{
			#[inline]
			fn $assign_method(&mut self, other: Polynomial<T>) { *self = std::mem::take(self).$method(&other) }
		}
		impl<T> $assign_op<&Polynomial<T>> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero $(+ $requirements<T, Output = T>)*
		{
			#[inline]
			fn $assign_method(&mut self, other: &Polynomial<T>) { *self = std::mem::take(self).$method(other) }
		}
		duplicate::duplicate! {
			[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]
			impl<T> $assign_op<primitive_type> for Polynomial<T>
			where T: $op<T, Output = T> + Clone + From<primitive_type> + Zero $(+ $requirements<T, Output = T>)*
			{
				#[inline]
				fn $assign_method(&mut self, other: primitive_type) { *self = std::mem::take(self).$method(Polynomial::from(std::vec![other.into()])) }
			}
		}
	};
}

#[macro_export(local_inner_macros)]
macro_rules! impl_op_all {
	($op:ident, $assign_op:ident, $method:ident, $assign_method:ident $(,$requirements:ident)*) => {
		impl_op_polynomial!($op, $method $(,$requirements:ident)*);
		impl_op_all_primitive!($op, $method $(,$requirements:ident)*);
		impl_assign_op!($op, $assign_op, $method, $assign_method $(,$requirements:ident)*);
	};
	($op:ident, $method:ident $(,$requirements:ident)*) => {
		impl_op_polynomial!($op, $method $(,$requirements:ident)*);
		impl_op_all_primitive!($op, $method $(,$requirements:ident)*);
	};
}
