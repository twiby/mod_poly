import sys
import argparse
import numpy as np
import poly_arithmetic as pa
from tqdm import tqdm
from symbolic_execution import start_interface

def bench_polynomial_product(deg = 100, nb_samples = 50):
	random_numbers = np.random.rand(deg, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	poly_1 = pa.Polynomial(coefs, deg)

	random_numbers = np.random.rand(deg, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	poly_2 = pa.Polynomial(coefs, deg)

	print("Computing polynomial multiplication with degree " + str(deg))
	for _ in tqdm(range(nb_samples)):
		poly_3 = poly_1 * poly_2
	print()

def bench_matrix_product(size = 100, nb_samples = 50):
	random_numbers = np.random.rand(size*size, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	m1 = pa.Matrix(coefs, size, size)

	random_numbers = np.random.rand(size*size, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	m2 = pa.Matrix(coefs, size, size)

	print("Computing complex matrix multiplication with size " + str(size) + "x" + str(size))
	for _ in tqdm(range(nb_samples)):
		m3 = m1 * m2
	print()

def bench_poly_matrix_product(deg = 100, size = 100, nb_samples = 50):
	random_numbers = np.random.rand(size*size*deg, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	polys = [pa.Polynomial(coefs[i*deg:(i+1)*deg], deg) for i in range(size*size)]
	m1 = pa.PolynomialMatrix(polys, size, size)

	random_numbers = np.random.rand(size*size*deg, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	polys = [pa.Polynomial(coefs[i*deg:(i+1)*deg], deg) for i in range(size*size)]
	m2 = pa.PolynomialMatrix(polys, size, size)

	print("Computing polynomial matrix multiplication with degree " + str(deg) + " and size " + str(size) + "x" + str(size))
	for _ in tqdm(range(nb_samples)):
		m3 = m1 * m2
	print()

def main(args):
	if args.interface:
		start_interface()
		sys.exit(0)

	print("Start benchmarking polynomial and matrix operations")
	print()

	bench_polynomial_product(deg = args.degree, nb_samples = args.nb_samples)
	bench_matrix_product(size = args.matrix, nb_samples = args.nb_samples)
	bench_poly_matrix_product(deg = args.poly_matrix_degree, size = args.matrix, nb_samples = args.nb_samples)

	sys.exit(0)

if __name__ == "__main__":
	parser = argparse.ArgumentParser(description="Helper for complex numbers and polynomials manipulation. If the --interface option is used, launches an interface, otherwise launch a benchmark of the Rust crate.")
	parser.add_argument('-i','--interface', action='store_true', help="Launches an interface for complex manipulation, ex: \"a = i+3i*5*(1+(3+6i))\" or \"a-(3+0.01*a)\"")
	parser.add_argument('-d','--degree', type=int, default=10000, help='Degree of the polynomials to be benchmarked (default 10000)')
	parser.add_argument('-n','--nb-samples', type=int, default=100, help="Number of operations in the benchmark (default 100)")
	parser.add_argument('-m','--matrix', type=int, default=100, help="Size of the matrices to be benchmarked (default 100)")
	parser.add_argument('--poly-matrix-degree',type=int, default=10, help="Degree of the polynomials inside the polynomial matrices to be benchmarked (default 10)")
	args = parser.parse_args()
	main(args)

	# bench_polynomial_product(30000)
