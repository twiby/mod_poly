import poly_arithmetic as pa

values = {}

allowed_characters_int = [
	"1",
	"2",
	"3",
	"4",
	"5",
	"6",
	"7",
	"8",
	"9",
	"0"
]

allowed_characters_float = allowed_characters_int + ['.']
allowed_characters_imag = allowed_characters_float + ['i']

def find_letter(letter, lst):
    return any(letter in word for word in lst)
def check_symbols(string, lst):
	if not all(find_letter(l, lst) for l in string):
		raise ValueError("Symbol not accepted in " + string)
def symbols_in_list(string, lst):
	if not all(find_letter(l, lst) for l in string):
		return False
	else:
		return True

def get_real(string):
	'''turns a string reprenting a float into a Complex object'''
	check_symbols(string, allowed_characters_float)
	if len(string.split('.')) > 2:
		raise ValueError("Too many points in " + string)

	return pa.Complex(float(string), 0)

def get_imag(string):
	'''turns a string reprenting an imaginary number into a Complex object'''
	check_symbols(string, allowed_characters_imag)
	if string == 'i':
		return pa.Complex(0,1)
	split = string.split("i")
	if len(split) != 2 or split[1] != '':
		raise ValueError("Weird imaginary: " + string)
	return pa.Complex(0, float(split[0]))

def get_unitary_complex(string):
	'''Turns a string representing a real or imaginary number into a Complex object'''
	check_symbols(string, allowed_characters_imag)
	if 'i' in string:
		return get_imag(string)
	else:
		return get_real(string)

def break_parenthesis(string):
	'''Recursively remove the parenthesis from the string, replacing each parenthesised term with a list. 
	The result of this operation is essentially a tree, where each node is a list and each leaf is a string'''

	### We only handle the first highest order parenthesis. The other ones are dealt with recursively
	opened_p = string.split('(')
	closed_p = string.split(')')
	opened_p_count = len(opened_p)
	closed_p_count = len(closed_p)
	if opened_p_count != closed_p_count:
		raise ValueError("Wrong parenthesis: " + string)
	if opened_p_count == 1:
		return [string]

	### This is the begging of the first parenthesised term, we want to find the right closing symbol
	first_opened_p = len(opened_p[0])
	sub_p_opened = 0
	for i in range(first_opened_p + 1, len(string)):
		if string[i] == '(':
			sub_p_opened += 1
		elif string[i] == ')' and sub_p_opened > 0:
			sub_p_opened -= 1
		elif string[i] == ')':
			first_closed_p = i
			break

	### Nested parenthesis are dealt with in this recursion
	ret = [break_parenthesis(string[first_opened_p+1:first_closed_p])]
	if first_opened_p > 0:
		ret = [string[:first_opened_p]] + ret
	if first_closed_p < len(string) - 1:
		### Subsequent parenthesis are dealt with in this recursion 
		ret += break_parenthesis(string[first_closed_p+1:])
	return ret

def remove_subtract(string):
	'''Recursively replace any "-" term with the terms "+", "-1" and "*", and returns the result as a list'''

	if not isinstance(string, str):
		raise TypeError
	if not '-' in string:
		return [string]
	new_lst = []
	first_subtract = len(string.split('-')[0])
	if first_subtract == 0:
		return ['+', pa.Complex(-1,0), '*'] + remove_subtract(string[1:])
	elif first_subtract == len(string)-1:
		return [string[:-1], '+', pa.Complex(-1,0), '*']
	else:
		return [string[:first_subtract], '+', pa.Complex(-1,0), '*']+remove_subtract(string[first_subtract+1:])

def separate_ops(string):
	'''Recursively split strings containing an operation to isolate operation symbols, and returns a list'''
	first_op = None
	for i in range(len(string)):
		if Evaluator.is_operation(string[i]):
			first_op = i
			break
	if first_op is None or len(string) == 1:
		return [string]
	elif first_op == 0:
		return [string[first_op]] + separate_ops(string[1:])
	elif first_op is len(string)-1:
		return [string[:-1], string[-1]]
	else:
		return [string[:first_op], string[first_op]] + separate_ops(string[first_op+1:])

def compute_operation(lst, symbol, fun):
	'''for every time "symbol" is found in lst, apply "fun" as a binary operator on the terms directly to the left and right'''
	new_lst = []
	skip_next = False
	for i in range(len(lst)):
		if skip_next:
			skip_next = False
			continue
		if lst[i] == symbol:
			new_lst[-1] = fun(new_lst[-1], lst[i+1])
			skip_next = True
		else:
			new_lst.append(lst[i])
	return new_lst

class Evaluator:
	'''Type that is meant to represent either a whole expression, or a parenthesised sub-expression.
	It contains every term in a list, with either numbers, operation symbol, or an Evaluator sub-expression.
	This list essentially forms an operation tree.'''

	def __init__(self, expression_lst):
		'''construct an expression tree where each number or operation is a leaf, 
		and each sub-expression is a node'''

		self.expressions = []
		if not isinstance(expression_lst, list):
			raise TypeError("Evaluator init: wring input type")
		
		### We replace each sub-list with an instantiation of Evaluator
		for e in expression_lst:
			if isinstance(e, list):
				self.expressions.append(Evaluator(e))
			elif isinstance(e, str):
				self.expressions.append(e)
			else:
				raise TypeError("Evaluator init: input must be a tree formed with only lists and strings")

		### Every subtract operation is replaced here "+ (-1) *"
		self.handle_subtract()

		### Expands any string terms like "3*5*" to a sequence "3","*","5","*"
		self.break_operations()

		### Some sanity checks are made to verify the validity of the string:
		### no 2 consecutive operations and no operation at the beginning or end of expression
		if self.operation_at_begin_or_end():
			raise ValueError("Evaluator: an expression starts or end with an operation")
		if self.consecutive_operation():
			raise ValueError("Evaluator: there are consecutive operations in the expression")

		### This is were we replace any Scalar value with a true numeric object, either from reading the string, or getting an existing value from "values"
		self.make_numbers()

	def __str__(exp):
		'''pretty print to actually see complex number values when printing'''
		ret = ""
		for e in exp:
			if Evaluator.is_operation(e):
				ret += str(e) + " "
			else:
				ret += "(" + str(e) + ") "
		return ret


	def evaluate(self):
		'''evaluate the expression into a single numeric object'''
		new_expressions = []

		### Replace any subexpression with an actual value
		for e in self.expressions:
			if isinstance(e, Evaluator):
				new_expressions.append(e.evaluate())
			else:
				new_expressions.append(e)

		### The order is where we implement the priority of multiplication over addition
		new_expressions = compute_operation(new_expressions, "*", pa.Complex.__mul__)
		new_expressions = compute_operation(new_expressions, "+", pa.Complex.__add__)

		if len(new_expressions) != 1:
			raise ValueError("Could not reduce every term when evaluating")

		return new_expressions[0]

	def handle_subtract(self):
		'''replace any "-" symbol with sumoething equivalent of "+(-1)*"'''
		new_expressions = []
		for e in self.expressions:
			if not isinstance(e, str):
				new_expressions.append(e)
			elif not '-' in e:
				new_expressions.append(e)
			else:
				new_expressions += remove_subtract(e)
		self.expressions = new_expressions

	def break_operations(self):
		'''Expands all string terms containing operations to isolate operation symbols'''
		new_expressions = []
		for e in self.expressions:
			if not isinstance(e, str):
				new_expressions.append(e)
			else:
				new_expressions += separate_ops(e)
		self.expressions = new_expressions

	def is_operation(char):
		return char == '+' or char == '*'

	def operation_at_begin_or_end(self):
		begin = self.expressions[0]
		if isinstance(begin, str) and Evaluator.is_operation(begin):
			return True
		end = self.expressions[-1]
		if isinstance(end, str) and Evaluator.is_operation(end):
			return True
		return False
	def consecutive_operation(self):
		last_item_was_operation = False
		for e in self.expressions:
			if Evaluator.is_operation(e):
				if last_item_was_operation:
					return True
				last_item_was_operation = True
			else:
				last_item_was_operation = False
		return False

	def make_numbers(self):
		'''Replace any purely scalar term with a numeric object'''

		new_expressions = []
		for e in self.expressions:
			if not isinstance(e, str):
				new_expressions.append(e)
			elif Evaluator.is_operation(e):
				new_expressions.append(e)
			elif symbols_in_list(e, allowed_characters_imag):
				new_expressions.append(get_unitary_complex(e))
			else:
				new_expressions.append(values[e])
		self.expressions = new_expressions

def evaluate_complex_number(string):
	'''Evaluate the expression in the string as a RHS if possible'''

	### First remove all spaces in the string
	string = "".join(string.split(" "))

	### Remove all parenthesis, converting them into sub lists
	string_lst = break_parenthesis(string)

	### Convert to a special types to handle sub-expressions easily
	evaluator = Evaluator(string_lst)
	return evaluator.evaluate()

def start_interface():
	print("Starting interface. Enter q to leave")
	while True:
		### User input
		expression = input(">> ")

		### Quitting
		if expression == 'q':
			print('exit')
			break

		### Definition
		elif '=' in expression:
			parts = expression.split('=')
			if len(parts) != 2:
				print("Error, try again.")

			name = "".join(parts[0].split(" "))
			if not name.isalpha():
				print("Names must have only letters")
				continue
			values[name] = evaluate_complex_number(parts[1])

		### Evaluation
		else:
			try:
				print(evaluate_complex_number(expression))
			except:
				print("Error, try again.")

if __name__ == "__main__":

	start_interface()



