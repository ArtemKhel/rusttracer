use rand::{
    distributions::{Distribution, Standard},
    random, Rng,
};
use strum_macros::EnumIter;

macro_rules! gen_axis_enums {
    ($name:ident { $( $axis:ident ),* }) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[derive(EnumIter)]
        pub enum $name {
            $( $axis ),*
        }
    }
}

gen_axis_enums!(Axis2 { X, Y });
gen_axis_enums!(Axis3 { X, Y, Z });
gen_axis_enums!(Axis4 { X, Y, Z, W });

macro_rules! axis_conversion {
    ($from:ident, $to:ident, { $($axis:ident),*}) => {
        impl From<$from> for $to {
            fn from(a: $from) -> Self{
                match a {
                $($from::$axis => $to::$axis),*,
                    _ => panic!("Axis out of bounds")
                }
            }
        }
        impl From<$to> for $from {
            fn from(a: $to) -> Self{
                match a {
                $($to::$axis => $from::$axis),*,
                    _ => panic!("Axis out of bounds")
                }
            }
        }
    };
}
axis_conversion!(Axis4, Axis3, {X,Y,Z});
axis_conversion!(Axis4, Axis2, {X,Y});
axis_conversion!(Axis3, Axis2, {X,Y});

#[macro_export]
macro_rules! impl_axis_index {
    ($type_name:ident, $axis_name:ident, $output:ty, $( ($axis:ident, $field:ident) ),* ) => {
        impl<T> Index<$axis_name> for $type_name<T>{
            type Output = $output;

            fn index(&self, index: $axis_name) -> &Self::Output {
                match index{
                    $( $axis_name::$axis => &self.$field ), *
                }
            }
        }
        impl<T> IndexMut<$axis_name> for $type_name<T>{
            fn index_mut(&mut self, index: $axis_name) -> &mut Self::Output {
                match index{
                    $( $axis_name::$axis => &mut self.$field ), *
                }
            }
        }
    };
}

impl Distribution<Axis2> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis2 {
        match random::<u32>() % 2 {
            0 => Axis2::X,
            1 => Axis2::Y,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Axis3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis3 {
        match random::<u32>() % 3 {
            0 => Axis3::X,
            1 => Axis3::Y,
            2 => Axis3::Z,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Axis4> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis4 {
        match random::<u32>() % 4 {
            0 => Axis4::X,
            1 => Axis4::Y,
            2 => Axis4::Z,
            3 => Axis4::W,
            _ => unreachable!(),
        }
    }
}
