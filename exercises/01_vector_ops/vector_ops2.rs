// 向量操作基础 - 练习2
//
// 在大型语言模型中，向量点积是计算相似度和注意力分数的基础操作。
// 本练习将帮助你学习如何在Rust中实现向量点积和其他常用操作。
//
// 任务：
// 1. 实现向量点积函数
// 2. 实现向量余弦相似度计算函数
// 3. 实现向量归一化函数

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    data: Vec<f32>,
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

    // TODO: 实现向量点积函数
    // 点积定义为：sum(a_i * b_i)
    pub fn dot_product(&self, other: &Vector) -> Result<f32, String> {
        // 在这里实现函数
        // 注意：两个向量长度必须相同，否则返回错误
        unimplemented!("实现向量点积计算");
    }

    // TODO: 实现向量余弦相似度计算函数
    // 余弦相似度定义为：dot(a, b) / (||a|| * ||b||)
    // 其中 ||a|| 表示向量a的L2范数
    pub fn cosine_similarity(&self, other: &Vector) -> Result<f32, String> {
        // 在这里实现函数
        // 提示：可以使用之前实现的点积函数和下面的l2_norm函数
        unimplemented!("实现向量余弦相似度计算");
    }

    // 计算向量的L2范数
    pub fn l2_norm(&self) -> f32 {
        self.data.iter().map(|&x| x * x).sum::<f32>().sqrt()
    }

    // TODO: 实现向量归一化函数
    // 归一化后的向量与原向量方向相同，但L2范数为1
    pub fn normalize(&self) -> Vector {
        // 在这里实现函数
        // 提示：每个元素除以向量的L2范数
        unimplemented!("实现向量归一化");
    }
}

// 不要修改测试代码
#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;

    #[test]
    fn test_dot_product() {
        let vec1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let vec2 = Vector::new(vec![4.0, 5.0, 6.0]);

        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(vec1.dot_product(&vec2).unwrap(), 32.0);

        // 测试长度不同的情况
        let vec3 = Vector::new(vec![1.0, 2.0]);
        assert!(vec1.dot_product(&vec3).is_err());
    }

    #[test]
    fn test_cosine_similarity() {
        let vec1 = Vector::new(vec![1.0, 0.0]);
        let vec2 = Vector::new(vec![0.0, 1.0]);

        // 正交向量的余弦相似度为0
        assert!(vec1.cosine_similarity(&vec2).unwrap().abs() < EPSILON);

        let vec3 = Vector::new(vec![1.0, 1.0]);
        let vec4 = Vector::new(vec![1.0, 1.0]);

        // 相同方向向量的余弦相似度为1
        assert!((vec3.cosine_similarity(&vec4).unwrap() - 1.0).abs() < EPSILON);

        let vec5 = Vector::new(vec![1.0, 2.0, 3.0]);
        let vec6 = Vector::new(vec![4.0, 5.0, 6.0]);

        // 计算余弦相似度
        let cos_sim = vec5.cosine_similarity(&vec6).unwrap();
        assert!((cos_sim - 0.9746318).abs() < 0.0001);
    }

    #[test]
    fn test_normalize() {
        let vec = Vector::new(vec![3.0, 4.0]);
        let normalized = vec.normalize();

        // 归一化后的向量长度应为1
        assert!((normalized.l2_norm() - 1.0).abs() < EPSILON);

        // 检查归一化后的向量方向是否正确
        assert!((normalized.data()[0] - 0.6).abs() < 0.0001); // 3/5 = 0.6
        assert!((normalized.data()[1] - 0.8).abs() < 0.0001); // 4/5 = 0.8
    }
}