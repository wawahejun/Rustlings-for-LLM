// 向量操作基础 - 练习3
//
// 在大型语言模型中，批量向量操作和SIMD优化对性能至关重要。
// 本练习将帮助你学习如何在Rust中实现高效的批量向量操作。
//
// 任务：
// 1. 实现向量批量乘法（element-wise multiplication）
// 2. 实现矩阵-向量乘法（常见于大模型的线性层）
// 3. 使用并行迭代器优化批量操作

use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    data: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    data: Vec<Vec<f32>>,
    rows: usize,
    cols: usize,
}

impl Vector {
    // 创建一个新的向量
    pub fn new(data: Vec<f32>) -> Self {
        Vector { data }
    }

    // 获取向量长度
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // 获取向量数据的引用
    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }

    // TODO: 实现向量的元素级乘法（element-wise multiplication）
    // 即：结果向量的每个元素是两个输入向量对应位置元素的乘积
    pub fn element_wise_mul(&self, other: &Vector) -> Result<Vector, String> {
        // 在这里实现函数
        // 注意：两个向量长度必须相同，否则返回错误
        unimplemented!("实现向量的元素级乘法");
    }

    // TODO: 实现向量的批量缩放（所有元素乘以同一个标量）
    // 提示：可以使用迭代器和map来实现
    pub fn scale(&self, scalar: f32) -> Vector {
        // 在这里实现函数
        unimplemented!("实现向量的批量缩放");
    }
}

// 为Vector实现Mul trait，使其支持向量与标量的乘法运算
// 例如：let scaled_vec = vec * 2.0;
impl Mul<f32> for &Vector {
    type Output = Vector;

    // TODO: 实现向量与标量的乘法运算
    fn mul(self, scalar: f32) -> Vector {
        // 在这里实现函数
        // 提示：可以调用上面实现的scale方法
        unimplemented!("实现向量与标量的乘法运算");
    }
}

impl Matrix {
    // 创建一个新的矩阵
    pub fn new(data: Vec<Vec<f32>>) -> Result<Self, String> {
        if data.is_empty() {
            return Err("矩阵不能为空".to_string());
        }

        let rows = data.len();
        let cols = data[0].len();

        // 检查所有行的长度是否相同
        if data.iter().any(|row| row.len() != cols) {
            return Err("所有行的长度必须相同".to_string());
        }

        Ok(Matrix { data, rows, cols })
    }

    // 获取矩阵的行数
    pub fn rows(&self) -> usize {
        self.rows
    }

    // 获取矩阵的列数
    pub fn cols(&self) -> usize {
        self.cols
    }

    // TODO: 实现矩阵-向量乘法
    // 这是大模型中最常见的操作之一，用于线性层的计算
    // 矩阵A(m×n)乘以向量x(n)得到向量b(m)
    pub fn mul_vector(&self, vec: &Vector) -> Result<Vector, String> {
        // 在这里实现函数
        // 注意：矩阵的列数必须等于向量的长度，否则返回错误
        // 提示：结果向量的每个元素是矩阵对应行与输入向量的点积
        unimplemented!("实现矩阵-向量乘法");
    }
}

// 不要修改测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_wise_mul() {
        let vec1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let vec2 = Vector::new(vec![4.0, 5.0, 6.0]);

        let result = vec1.element_wise_mul(&vec2).unwrap();
        assert_eq!(result.data(), &vec![4.0, 10.0, 18.0]);

        // 测试长度不同的情况
        let vec3 = Vector::new(vec![1.0, 2.0]);
        assert!(vec1.element_wise_mul(&vec3).is_err());
    }

    #[test]
    fn test_scale() {
        let vec = Vector::new(vec![1.0, 2.0, 3.0]);
        let scaled = vec.scale(2.0);
        assert_eq!(scaled.data(), &vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_vector_scalar_mul() {
        let vec = Vector::new(vec![1.0, 2.0, 3.0]);
        let scaled = &vec * 3.0;
        assert_eq!(scaled.data(), &vec![3.0, 6.0, 9.0]);
    }

    #[test]
    fn test_matrix_vector_mul() {
        // 创建一个2×3矩阵
        let matrix = Matrix::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ])
        .unwrap();

        // 创建一个长度为3的向量
        let vector = Vector::new(vec![2.0, 3.0, 4.0]);

        // 计算矩阵-向量乘法
        // [1.0, 2.0, 3.0] · [2.0, 3.0, 4.0] = 1*2 + 2*3 + 3*4 = 2 + 6 + 12 = 20
        // [4.0, 5.0, 6.0] · [2.0, 3.0, 4.0] = 4*2 + 5*3 + 6*4 = 8 + 15 + 24 = 47
        let result = matrix.mul_vector(&vector).unwrap();
        assert_eq!(result.data(), &vec![20.0, 47.0]);

        // 测试维度不匹配的情况
        let vector2 = Vector::new(vec![1.0, 2.0]);
        assert!(matrix.mul_vector(&vector2).is_err());
    }
}

// 挑战：尝试使用Rust的并行迭代器（例如rayon）来优化上述操作
// 提示：在实际项目中，可以添加特性标志来有条件地启用并行计算
// #[cfg(feature = "parallel")]
// use rayon::prelude::*;
//
// 然后修改相关函数，使用并行迭代器：
// self.data.par_iter().zip(other.data.par_iter()).map(...).collect()