// 在大型语言模型中，注意力机制是一个核心组件。
// 在这个练习中，我们将实现注意力分数的计算，包括向量点积和softmax操作。

// TODO: 实现向量点积函数
// 输入两个等长的向量，计算它们的点积
fn vector_dot_product(v1: &[f32], v2: &[f32]) -> f32 {
    assert_eq!(v1.len(), v2.len(), "向量长度必须相同");
    v1.iter()
        .zip(v2.iter())
        .map(|(&a, &b)| a * b)
        .sum()
}


// TODO: 实现注意力分数计算函数
// 输入query和key向量，计算注意力分数
fn compute_attention_score(query: &[f32], key: &[f32]) -> f32 {
    vector_dot_product(query, key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;

    const EPSILON: f32 = 1e-5;

    #[test]
    fn test_vector_dot_product() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        let result = vector_dot_product(&v1, &v2);
        assert!((result - 32.0).abs() < EPSILON, "向量点积计算错误");
    }


    #[test]
    fn test_attention_score() {
        let query = vec![1.0, 1.0, 1.0];
        let key = vec![1.0, 0.0, 1.0];
        let score = compute_attention_score(&query, &key);
        
        // 预期结果：点积后的结果为2.0
        assert!((score - 2.0).abs() < EPSILON, "注意力分数计算错误");
    }

}