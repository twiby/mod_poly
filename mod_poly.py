import numpy as np
import poly_arithmetic as pa

def bench_polynomial_product(N = 100):
	random_numbers = np.random.rand(N, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	poly_1 = pa.Polynomial( coefs, N)

	random_numbers = np.random.rand(N, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	poly_2 = pa.Polynomial( coefs, N)

	poly_3 = poly_1 * poly_2


if __name__ == "__main__":
	a = pa.Complex(1, 1)
	b = pa.Complex(1, 1)

	print(a, b)
	print(a[0], a[1])
	a[1] = 100

	print(a + b)
	print(a * b)

	poly_1 = pa.Polynomial([pa.Complex(1,0), pa.Complex(0,2), pa.Complex(1,1)], 3)
	poly_2 = pa.Polynomial([pa.Complex(1,1), pa.Complex(1,0), pa.Complex(0,2)], 3)
	print()
	print(poly_1 + poly_2)
	print(poly_1 * poly_2)
	poly_1[2] = pa.Complex(1,10)
	print(poly_1[2])

	m1 = pa.Matrix([pa.Complex(1,0), pa.Complex(0,2), pa.Complex(1,1), pa.Complex(1,1), pa.Complex(1,0), pa.Complex(0,2)], 2, 3)
	m2 = pa.Matrix([pa.Complex(1,0), pa.Complex(0,2), pa.Complex(1,1), pa.Complex(1,1), pa.Complex(1,0), pa.Complex(0,2)], 3, 2)
	print()
	print(m1)
	print(m1[(0,1)])
	m1[(0,1)] = pa.Complex(1, 2)
	print(m1[(0,1)])
	print(m1 + m1)
	print(m1 * m2)

	a = pa.Complex(1, 0)
	b = pa.Complex(0, 2)
	c = pa.Complex(1, 1)
	poly1 = pa.Polynomial([a, b, c], 3)
	poly2 = pa.Polynomial([c, a, b], 3)

	m1 = pa.PolynomialMatrix([poly1, poly2], 1, 2)
	m2 = pa.PolynomialMatrix([poly1, poly2], 1, 2)
	print(m1)
	print(m1 + m2)
	m2 = pa.PolynomialMatrix([poly1, poly1], 2, 1)
	print(m2[(0,0)])
	m2[(0,0)] = poly2
	print(m1 * m2)

	bench_polynomial_product(30000)
