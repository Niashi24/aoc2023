use std::fmt::Debug;
use std::iter::{Product, Sum};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Range, Sub};
use nalgebra::RealField;
use num::{Float, Num, Zero};
use num::traits::FloatConst;

pub(crate) fn gradient_descent<const N: usize, FN>(
    start: [f64; N],
    func: FN,
    max_iterations: usize,
) -> ([f64; N], f64)
    where
        FN: Fn([f64; N]) -> f64,
{
    const LEARNING_RATE: f64 = 1e0;
    const EPSILON: f64 = 1e0;
    const TOLERANCE: f64 = 1e-4;

    let mut current_point = start;
    let mut current_value = func(current_point);
    
    // while current_value > 1e-4 {

    for _ in 0..max_iterations {
        let mut gradient = [0.0; N];

        // Calculate the gradient
        for i in 0..N {
            let mut perturbed_point = current_point;
            perturbed_point[i] += EPSILON;
            let perturbed_value = func(perturbed_point);
            gradient[i] = (perturbed_value - current_value) / EPSILON;
        }

        // Update the current point using the gradient and learning rate
        for i in 0..N {
            current_point[i] -= LEARNING_RATE * gradient[i];
        }

        // Check for convergence
        let new_value = func(current_point);
        if (current_value - new_value).abs() < TOLERANCE {
            return (current_point, new_value);
        }
        // 
        // println!("{:?}", current_point);
        // if current_point.iter().any(|i| i.is_nan()) {
        //     break;
        // }

        current_value = new_value;
    }

    // Return the current point and its corresponding function value
    (current_point, current_value)
}

// #[test]
pub(crate) fn test_grad_descent() {
    fn test(x: [f64; 1]) -> f64 {
        x[0] * x[0] - 4.0 * x[0] + 1.0
    }
    
    let r = gradient_descent([100.0], |x| test(x), 10000);
    println!("{:?}", r);
    
    fn test_2(x: &[f64; 2]) -> f64 {
        let [x, y] = x;
        const a: f64 = -2.5;
        const b: f64 = 3.0;
        const c: f64 = 1.95;
        (x - a).powi(2) + (y - b).powi(2) + c
    }
    

    // let r = gradient_descent([1.0, 0.0], |x| test_2(x), 1000000);
    // println!("{:?}", r);

    let ackley_fn = |&[x, y]: &[f64; 2]| {
        -20.0 * f64::exp(-0.2 * f64::sqrt(0.5 * (x * x + y * y)))
        - f64::exp(0.5 * (f64::cos(2.0 * f64::PI() * x) + f64::cos(2.0 * f64::PI() * y)))
        + f64::e() + 20.0
    };
    let target = Target {
        func: &test_2,
        pd: Default::default(),
    };
    
    let y = target.steepest_descent(&[1.0, 0.0], 100, 0.001)
        .unwrap();
    
    println!("{:?}", find_minimum(test_2, None));
}

pub fn find_minimum<const N: usize, FN>(func: FN, start: Option<[f64; N]>) -> ([f64; N], f64)
where
    FN: Fn(&[f64; N]) -> f64,
{
    let target = Target {
        func: &func,
        pd: Default::default(),
    };
    
    let start = start.unwrap_or([0.0; N]);
    
    let y = target.steepest_descent(&start, 10000, 1e-9).unwrap();
    (y, func(&y))
}

#[derive(Debug)]
pub struct Vector<const N: usize, X>([X; N]);

impl<const N: usize, X> Mul<&Vector<N, X>> for &Vector<N, X>
where
    X: Copy + Mul<X, Output = X> + Sum<X>,
{
    type Output = X;

    fn mul(self, rhs: &Vector<N, X>) -> Self::Output {
        self.0.iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| *a * *b)
            .sum()
    }
}


impl<const N: usize, X> Mul<X> for &Vector<N, X>
where
    X: Copy + Mul<X, Output = X>,
{
    type Output = Vector<N, X>;
    fn mul(self, rhs: X) -> Self::Output {
        Vector(self.0.map(|e| e * rhs))
    }
}

impl<const N: usize, X> Sub<Vector<N, X>> for &Vector<N, X>
where
    X: Copy + Sub<X, Output = X>,
{
    type Output = Vector<N, X>;
    fn sub(self, rhs: Vector<N, X>) -> Self::Output {
        let mut output = self.0;
        output.iter_mut().enumerate()
            .zip(rhs.0)
            .for_each(|((i, x), y)| *x = *x - y );
        Vector(output)
    }
}

pub struct Target<const N: usize, X, FN>
where
    FN: Fn(&[X; N]) -> X,
    // GD: Fn(&[X; N]) -> [X; N],
{
    pub func: FN,
    // pub grad: Option<GD>,
    pd: PhantomData<X>,
}

pub trait SteepestDescent<const N: usize, X> {
    fn steepest_descent(
        &self,
        x_0: &[X; N],
        max_iter: usize,
        tolerance: X,
    ) -> Result<[X; N], String>;
    
    fn optimize_alpha(&self, x: &Vector<N, X>, grad: &Vector<N, X>, norm: X, tolerance: X) -> f64;
}

impl<const N: usize, FN> SteepestDescent<N, f64> for Target<N, f64, FN>
where
    FN: Fn(&[f64; N]) -> f64,
{
    fn steepest_descent(
        &self,
        x_0: &[f64; N],
        max_iters: usize,
        tolerance: f64
    ) -> Result<[f64; N], String> {
        let mut x = Vector(x_0.clone());
        for _ in 0..max_iters {
            let grad = Vector(gradient(&self.func, &x.0, 0.001));
            let norm = f64::sqrt(&grad * &grad);
            if norm < tolerance {
                return Ok(x.0);
            }
            let alpha = self.optimize_alpha(&x, &grad, norm, tolerance);
            if alpha.abs() < tolerance {
                return Ok(x.0);
            }
            x = &x - &grad * (alpha / norm);
        }
        Err(String::from("exceeded maximal iterations"))
    }

    fn optimize_alpha(&self, x: &Vector<N, f64>, grad: &Vector<N, f64>, norm: f64, tolerance: f64) -> f64 {
        let mut beta = 1.0;
        let v_x = (self.func)(&x.0);
        let g = |alpha| (self.func)(&(x - grad * (alpha / norm)).0);
        beta = loop {
            let v = g(beta);
            if v < v_x {
                break beta;
            }
            if f64::abs(beta) < tolerance {
                return 0.0;
            }
            beta /= 2.0;
        };
        
        let c = newton(&[
            (0.0, v_x),
            (beta / 2.0, g(beta / 2.0)),
            (beta, g(beta))
        ]);
        
        let alpha = 1.0 / (2.0 * c[2]) * (c[2] * beta / 2.0 - c[1]);
        if g(alpha) < g(beta) {
            alpha
        } else {
            beta
        }
    }
}

fn first_order<FN, X>(f: &FN, x: X, h: X) -> X
where
    FN: Fn(X) -> X,
    X: Sub<X, Output = X> + Add<X, Output = X> + Copy + Div<X, Output = X>
{
    (f(x + h) - f(x)) / h
}

fn partial<const N: usize, FN, X>(f: &FN, x: &[X; N], i: usize, h: X) -> X
where
    FN: Fn(&[X; N]) -> X,
    X: Sub<X, Output = X> + Add<X, Output = X> + Copy + Div<X, Output = X>,
{
    let g = |z: X| {
        let mut x_copy = x.clone();
        x_copy[i] = z;
        f(&x_copy)
    };
    
    first_order(&g, x[i], h)
}

fn gradient<const N: usize, FN, X>(f: &FN, x: &[X; N], h: X) -> [X; N]
where
    FN: Fn(&[X; N]) -> X,
    X: Sub<X, Output = X> + Add<X, Output = X> + Copy + Div<X, Output = X>,
{
    let mut out = x.clone();
    
    out.iter_mut().enumerate()
        .for_each(|(i, x_i)| *x_i = partial(f, x, i, h));
    
    out
}

fn newton<const N: usize, X>(points: &[(X, X); N]) -> [X; N]
    where
        X: Copy
        + Sum
        + Product
        + Mul<X, Output = X>
        + Sub<X, Output = X>
        + Neg<Output = X>
        + Div<X, Output = X>
        + 'static
        + Debug
{
    let mut coefs = vec![];
    rec_newton(points, &mut coefs);
    coefs.try_into().unwrap()
}

fn rec_newton<const N: usize, X>(points: &[(X, X); N], coefs: &mut Vec<X>)
    where
        X: Copy
        + Sum
        + Product
        + Mul<X, Output = X>
        + Sub<X, Output = X>
        + Neg<Output = X>
        + Div<X, Output = X>
        + 'static,
{
    let k = coefs.len();
    if points.len() > k {
        let (x, y) = points[k];
        let s = points[..k]
            .iter().map(|(p_x, _)| x - *p_x)
            .product();
        coefs.push(y - evaluate::<N, X>(x, coefs, points) / s);
        rec_newton::<N, X>(points, coefs);
    }
}

fn evaluate<const N: usize, X>(x: X, coefs: &Vec<X>, points: &[(X, X); N]) -> X
where
    X: Copy
        + Sum
        + Product
        + Mul<X, Output = X>
        + Sub<X, Output = X>
        + Neg<Output = X>
        + Div<X, Output = X>
        + 'static,
{
    coefs.iter()
        .enumerate()
        .map(|(k, a_k)| {
            if k == 0 {
                *a_k
            } else {
                *a_k * points[..k].iter()
                    .map(|(p_x, _)| x - *p_x)
                    .product()
            }
        }).sum()
}