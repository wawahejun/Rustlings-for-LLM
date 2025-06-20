// 在Transformer架构中，多头注意力是一个核心组件。
// 在这个练习中，我们将实现多头注意力中的关键张量运算。

// 实现张量形状变换函数，用于多头注意力中的头部分割
// 输入张量形状为 [batch_size, seq_len, hidden_size]
// 输出张量形状为 [batch_size, num_heads, seq_len, head_size]
// 其中 hidden_size = num_heads * head_size
fn reshape_for_attention(
    input: &[Vec<Vec<f32>>],
    num_heads: usize
) -> Vec<Vec<Vec<Vec<f32>>>> {
    let batch_size = input.len();
    let seq_len = input[0].len();
    let hidden_size = input[0][0].len();
    let head_size = hidden_size / num_heads;
    
    input.iter()
        .map(|batch| {
            batch.iter()
                .map(|seq| {
                    seq.chunks(head_size)
                        .map(|chunk| chunk.to_vec())
                        .collect::<Vec<Vec<f32>>>()
                })
                .collect::<Vec<Vec<Vec<f32>>>>()
        })
        .map(|batch_heads| {
            (0..num_heads)
                .map(|h| {
                    (0..seq_len)
                        .map(|s| batch_heads[s][h].clone())
                        .collect::<Vec<Vec<f32>>>()
                })
                .collect::<Vec<Vec<Vec<f32>>>>()
        })
        .collect()
}

// 实现张量转置函数
// 输入张量形状为 [batch_size, num_heads, seq_len, head_size]
// 输出张量形状为 [batch_size, num_heads, head_size, seq_len]
fn transpose_for_scores(
    x: &[Vec<Vec<Vec<f32>>>]
) -> Vec<Vec<Vec<Vec<f32>>>> {
    x.iter()
        .map(|batch| {
            batch.iter()
                .map(|head| {
                    let seq_len = head.len();
                    let head_size = head[0].len();
                    
                    (0..head_size)
                        .map(|h| {
                            (0..seq_len)
                                .map(|s| head[s][h])
                                .collect::<Vec<f32>>()
                        })
                        .collect::<Vec<Vec<f32>>>()
                })
                .collect::<Vec<Vec<Vec<f32>>>>()
        })
        .collect()
}

// 实现注意力分数的批量计算
// query形状: [batch_size, num_heads, seq_len, head_size]
// key形状: [batch_size, num_heads, head_size, seq_len]
// 输出形状: [batch_size, num_heads, seq_len, seq_len]
fn batch_matmul(
    query: &[Vec<Vec<Vec<f32>>>],
    key: &[Vec<Vec<Vec<f32>>>]
) -> Vec<Vec<Vec<Vec<f32>>>> {
    let batch_size = query.len();
    let num_heads = query[0].len();
    let seq_len_q = query[0][0].len();
    let head_size = query[0][0][0].len();
    let seq_len_k = key[0][0][0].len();
    
    assert_eq!(head_size, key[0][0].len(), "Head size mismatch");
    
    (0..batch_size)
        .map(|b| {
            (0..num_heads)
                .map(|h| {
                    let q = &query[b][h];
                    let k = &key[b][h];
                    
                    (0..seq_len_q)
                        .map(|i| {
                            (0..seq_len_k)
                                .map(|j| {
                                    q[i].iter()
                                        .zip(k[j].iter())
                                        .map(|(&a, &b)| a * b)
                                        .sum::<f32>()
                                })
                                .collect::<Vec<f32>>()
                        })
                        .collect::<Vec<Vec<f32>>>()
                })
                .collect::<Vec<Vec<Vec<f32>>>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    #[test]
    fn test_reshape_for_attention() {
        // 创建一个简单的测试用例
        // batch_size = 1, seq_len = 2, hidden_size = 4
        let input = vec![
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0]
            ]
        ];
        let num_heads = 2; // head_size将为2

        let result = reshape_for_attention(&input, num_heads);

        // 检查输出维度
        assert_eq!(result.len(), 1); // batch_size
        assert_eq!(result[0].len(), 2); // num_heads
        assert_eq!(result[0][0].len(), 2); // seq_len
        assert_eq!(result[0][0][0].len(), 2); // head_size

        // 检查值是否正确重排
        // 第一个头应该包含原始向量的前半部分
        assert!((result[0][0][0][0] - 1.0).abs() < EPSILON);
        assert!((result[0][0][0][1] - 2.0).abs() < EPSILON);
        assert!((result[0][0][1][0] - 5.0).abs() < EPSILON);
        assert!((result[0][0][1][1] - 6.0).abs() < EPSILON);

        // 第二个头应该包含原始向量的后半部分
        assert!((result[0][1][0][0] - 3.0).abs() < EPSILON);
        assert!((result[0][1][0][1] - 4.0).abs() < EPSILON);
        assert!((result[0][1][1][0] - 7.0).abs() < EPSILON);
        assert!((result[0][1][1][1] - 8.0).abs() < EPSILON);
    }

    #[test]
    fn test_transpose_for_scores() {
        let input = vec![
            vec![
                vec![
                    vec![1.0, 2.0],
                    vec![3.0, 4.0]
                ],
                vec![
                    vec![5.0, 6.0],
                    vec![7.0, 8.0]
                ]
            ]
        ];

        let result = transpose_for_scores(&input);

        // 检查输出维度
        assert_eq!(result.len(), 1); // batch_size
        assert_eq!(result[0].len(), 2); // num_heads
        assert_eq!(result[0][0].len(), 2); // head_size
        assert_eq!(result[0][0][0].len(), 2); // seq_len

        // 检查转置是否正确
        assert!((result[0][0][0][0] - 1.0).abs() < EPSILON);
        assert!((result[0][0][0][1] - 3.0).abs() < EPSILON);
        assert!((result[0][0][1][0] - 2.0).abs() < EPSILON);
        assert!((result[0][0][1][1] - 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_batch_matmul() {
        let query = vec![
            vec![
                vec![
                    vec![1.0, 2.0],
                    vec![3.0, 4.0]
                ]
            ]
        ];
        let key = vec![
            vec![
                vec![
                    vec![1.0, 3.0],
                    vec![2.0, 4.0]
                ]
            ]
        ];

        let result = batch_matmul(&query, &key);

        // 检查输出维度
        assert_eq!(result.len(), 1); // batch_size
        assert_eq!(result[0].len(), 1); // num_heads
        assert_eq!(result[0][0].len(), 2); // seq_len
        assert_eq!(result[0][0][0].len(), 2); // seq_len

        // 检查矩阵乘法结果
        // [1.0, 2.0] · [1.0, 3.0] = 1.0 * 1.0 + 2.0 * 3.0 = 7.0
        assert!((result[0][0][0][0] - 7.0).abs() < EPSILON);
        // [1.0, 2.0] · [2.0, 4.0] = 1.0 * 2.0 + 2.0 * 4.0 = 10.0
        assert!((result[0][0][0][1] - 10.0).abs() < EPSILON);
        // [3.0, 4.0] · [1.0, 3.0] = 3.0 * 1.0 + 4.0 * 3.0 = 15.0
        assert!((result[0][0][1][0] - 15.0).abs() < EPSILON);
        // [3.0, 4.0] · [2.0, 4.0] = 3.0 * 2.0 + 4.0 * 4.0 = 22.0
        assert!((result[0][0][1][1] - 22.0).abs() < EPSILON);
    }
}