
import tensorflow as tf

def softmax_cross_entropy_with_logits(y_true, y_pred):
	"""我们要让p和pi的loss越小越好"""
	p = y_pred
	pi = y_true

	zero = tf.zeros_like(pi, dtype=tf.float32)
	# zero = tf.zeros(shape = tf.shape(pi), dtype=tf.float32)
	where = tf.equal(pi, zero) 		# 对pi, p的系列决策中的每一个做是否相等的判别

	negatives = tf.fill(tf.shape(pi), -100.0) 
	p = tf.where(where, negatives, p)

	loss = tf.nn.softmax_cross_entropy_with_logits(labels = pi, logits = p)

	return loss


