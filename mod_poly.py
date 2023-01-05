import sys
import argparse
import numpy as np
import poly_arithmetic as pa
from symbolic_execution import start_interface

def bench_polynomial_product(N = 100):
	random_numbers = np.random.rand(N, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	poly_1 = pa.Polynomial( coefs, N)

	random_numbers = np.random.rand(N, 2)
	coefs = [pa.Complex(i,j) for i, j in random_numbers]
	poly_2 = pa.Polynomial( coefs, N)

	poly_3 = poly_1 * poly_2

def main(args):
	if args.interface:
		start_interface()
		sys.exit(0)
	sys.exit(0)

if __name__ == "__main__":
	parser = argparse.ArgumentParser(description="Helper for complex numbers and polynomials manipulation.")
	parser.add_argument('-i','--interface', action='store_true', help="Launches an interface for complex manipulation, ex: \"a = i+3i*5*(1+(3+6i))\" or \"a-(3+0.01*a)\"")
	# parser.add_argument('-w','--wordCheck', type=str, help="returns all words formed with these letters")
	# parser.add_argument('-p','--players', type=int, help="number of players")
	# parser.add_argument('--scrabbleStats', action='store_true', help="makes stats about probability of having a scrabble")
	# parser.add_argument('-a','--auto', action='store_true', help="launches a game with automatic players")
	# parser.add_argument('-s','--show', action='store_true', help="shows current best word")
	args = parser.parse_args()
	main(args)

	# bench_polynomial_product(30000)
