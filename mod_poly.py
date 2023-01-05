import numpy as np
import poly_arithmetic as pa

def bench_polynomial_product(N = 100):
	random_numbers = np.random.rand(N, 2)
	poly_1 = pa.Polynomial( list(map(tuple, random_numbers)), N)
	random_numbers = np.random.rand(N, 2)
	poly_2 = pa.Polynomial( list(map(tuple, random_numbers)), N)

	poly_3 = poly_1 * poly_2


if __name__ == "__main__":
	a = pa.Complex(1, 1)
	b = pa.Complex(1, 1)

	print(a, b)

	print(a + b)
	print(a * b)

	poly_1 = pa.Polynomial([(1,0), (0,2), (1,1)], 3)
	poly_2 = pa.Polynomial([(1,1), (1,0), (0,2)], 3)
	print(poly_1)
	print(poly_1 + poly_2)
	print(poly_1 * poly_2)

	m1 = pa.Matrix([(1,0), (0,2), (1,1), (1,1), (1,0), (0,2)], 2, 3)
	m2 = pa.Matrix([(1,0), (0,2), (1,1), (1,1), (1,0), (0,2)], 3, 2)
	print(m1)
	print(m1 + m1)
	print(m1 * m2)

	bench_polynomial_product(30000)
