// 张量计算 - 练习1
//
// 张量是大型语言模型的核心数据结构，它是向量和矩阵的高维推广。
// 本练习将帮助你学习如何在Rust中创建和操作张量。
//
// 任务：
// 1. 实现张量的创建函数
// 2. 实现张量元素的访问和修改
// 3. 实现张量形状的操作（reshape、transpose）

use std::fmt;

// 张量结构体，使用Vec<f32>存储数据，shape表示张量的维度
#[derive(Clone, PartialEq)]
pub struct Tensor {
    data: Vec<f32>,
    shape: Vec<usize>,
}

impl Tensor {
    // TODO: 实现创建指定形状的零张量的函数
    pub fn zeros(shape: Vec<usize>) -> Self {
        // 在这里实现函数
        // 提示：计算总元素数，然后创建对应大小的零向量
        unimplemented!("创建一个形状为 {:?} 的零张量", shape);
    }

    // TODO: 实现创建指定形状的全1张量的函数
    pub fn ones(shape: Vec<usize>) -> Self {
        // 在这里实现函数
        unimplemented!("创建一个形状为 {:?} 的全1张量", shape);
    }

    // 获取张量的形状
    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    // 获取张量的总元素数
    pub fn size(&self) -> usize {
        self.data.len()
    }

    // TODO: 实现获取张量在指定索引处的值
    // 例如，对于形状为[2,3]的张量，get(&[1,2])应返回第1行第2列的元素
    pub fn get(&self, indices: &[usize]) -> Result<f32, String> {
        // 在这里实现函数
        // 提示：需要将多维索引转换为一维数组的索引
        unimplemented!("获取索引 {:?} 处的值", indices);
    }

    // TODO: 实现设置张量在指定索引处的值
    pub fn set(&mut self, indices: &[usize], value: f32) -> Result<(), String> {
        // 在这里实现函数
        unimplemented!("设置索引 {:?} 处的值为 {}", indices, value);
    }

    // TODO: 实现张量的reshape操作
    // 注意：新形状的总元素数必须与原形状相同
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Tensor, String> {
        // 在这里实现函数
        unimplemented!("将张量从形状 {:?} 重塑为 {:?}", self.shape, new_shape);
    }

    // TODO: 实现2D张量的转置操作
    // 注意：此函数仅适用于2D张量（矩阵）
    pub fn transpose_2d(&self) -> Result<Tensor, String> {
        // 在这里实现函数
        // 提示：检查张量是否为2D，然后交换行和列
        unimplemented!("转置2D张量");
    }
}

// 为Tensor实现Debug trait，方便打印和调试
impl fmt::Debug for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tensor(shape={:?}, data={:?})", self.shape, self.data)
    }
}

// 不要修改测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let tensor = Tensor::zeros(vec![2, 3]);
        assert_eq!(tensor.shape(), &vec![2, 3]);
        assert_eq!(tensor.size(), 6);
        for i in 0..6 {
            assert_eq!(tensor.data[i], 0.0);
        }
    }

    #[test]
    fn test_ones() {
        let tensor = Tensor::ones(vec![2, 2]);
        assert_eq!(tensor.shape(), &vec![2, 2]);
        assert_eq!(tensor.size(), 4);
        for i in 0..4 {
            assert_eq!(tensor.data[i], 1.0);
        }
    }

    #[test]
    fn test_get_set() {
        let mut tensor = Tensor::zeros(vec![2, 3]);

        // 设置值
        tensor.set(&[0, 1], 5.0).unwrap();
        tensor.set(&[1, 2], 7.0).unwrap();

        // 获取值
        assert_eq!(tensor.get(&[0, 1]).unwrap(), 5.0);
        assert_eq!(tensor.get(&[1, 2]).unwrap(), 7.0);
        assert_eq!(tensor.get(&[0, 0]).unwrap(), 0.0);

        // 测试索引越界
        assert!(tensor.get(&[2, 0]).is_err());
        assert!(tensor.set(&[0, 3], 10.0).is_err());
    }

    #[test]
    fn test_reshape() {
        let tensor = Tensor::ones(vec![2, 3]);
        let reshaped = tensor.reshape(vec![3, 2]).unwrap();
        assert_eq!(reshaped.shape(), &vec![3, 2]);
        assert_eq!(reshaped.size(), 6);

        // 测试元素数不匹配的情况
        assert!(tensor.reshape(vec![2, 2]).is_err());
    }

    #[test]
    fn test_transpose_2d() {
        let mut tensor = Tensor::zeros(vec![2, 3]);
        tensor.set(&[0, 0], 1.0).unwrap();
        tensor.set(&[0, 1], 2.0).unwrap();
        tensor.set(&[0, 2], 3.0).unwrap();
        tensor.set(&[1, 0], 4.0).unwrap();
        tensor.set(&[1, 1], 5.0).unwrap();
        tensor.set(&[1, 2], 6.0).unwrap();

        let transposed = tensor.transpose_2d().unwrap();
        assert_eq!(transposed.shape(), &vec![3, 2]);
        assert_eq!(transposed.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(transposed.get(&[1, 0]).unwrap(), 2.0);
        assert_eq!(transposed.get(&[2, 0]).unwrap(), 3.0);
        assert_eq!(transposed.get(&[0, 1]).unwrap(), 4.0);
        assert_eq!(transposed.get(&[1, 1]).unwrap(), 5.0);
        assert_eq!(transposed.get(&[2, 1]).unwrap(), 6.0);

        // 测试非2D张量
        let tensor_3d = Tensor::zeros(vec![2, 2, 2]);
        assert!(tensor_3d.transpose_2d().is_err());
    }
}