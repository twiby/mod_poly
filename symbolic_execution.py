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
def remove_all_spaces(string):
	ret = string.split(" ")
	return "".join(ret)
def remove_parenthesis_around(string):
	ret = string
	if ret[0] == '(':
		ret = ret[1:]
		if ret[-1] == ')':
			ret = ret[:-1]
		else:
			raise ValueError("Wrong parenthesis")
	elif ret[-1] == ')':
		raise ValueError("Wrong parenthesis")
	return ret


def get_float(string):
	check_symbols(string, allowed_characters_float)
	if len(string.split('.')) > 2:
		raise ValueError("Too many points in " + string)

	return pa.Complex(float(string), 0)

def get_imag(string):
	check_symbols(string, allowed_characters_imag)
	if string == 'i':
		return pa.Complex(0,1)
	split = string.split("i")
	if len(split) != 2 or split[1] != '':
		raise ValueError("Weird imaginary: " + string)
	return pa.Complex(0, float(split[0]))

def get_unitary_complex(string):
	check_symbols(string, allowed_characters_imag)
	if 'i' in string:
		return get_imag(string)
	else:
		return get_float(string)

def break_parenthesis(string):
	#TODO check symbols
	opened_p = string.split('(')
	closed_p = string.split(')')
	opened_p_count = len(opened_p)
	closed_p_count = len(closed_p)
	if opened_p_count != closed_p_count:
		raise ValueError("Wrong parenthesis: " + string)
	if opened_p_count == 1:
		return [string]
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
	ret = [break_parenthesis(string[first_opened_p+1:first_closed_p])]
	if first_opened_p > 0:
		ret = [string[:first_opened_p]] + ret
	if first_closed_p < len(string) - 1:
		ret += break_parenthesis(string[first_closed_p+1:])
	return ret

def remove_subtract(string):
	if not isinstance(string, str):
		raise TypeError
	if not '-' in string:
		return [string]
	new_lst = []
	first_subtract = len(string.split('-')[0])
	if first_subtract == 0:
		return ['+', pa.Complex(-1,0),'*']+remove_subtract(string[1:])
	elif first_subtract == len(string)-1:
		return [string[:-1], '+',pa.Complex(-1,0),'*']
	else:
		return [string[:first_subtract], '+', pa.Complex(-1,0),'*']+remove_subtract(string[first_subtract+1:])

def separate_ops(string):
	first_op = None
	for i in range(len(string)):
		if string[i] == '+' or string[i] == '*':
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

def compute_mult(lst):
	if not isinstance(lst, list):
		raise TypeError
	new_lst = []
	skip_next = False
	for i in range(len(lst)):
		if skip_next:
			skip_next = False
			continue
		if lst[i] == '*':
			new_lst[-1] *= lst[i+1]
			skip_next = True
		else:
			new_lst.append(lst[i])
	return new_lst

def compute_add(lst):
	if not isinstance(lst, list):
		raise TypeError
	new_lst = []
	skip_next = False
	for i in range(len(lst)):
		if skip_next:
			skip_next = False
			continue
		if lst[i] == '+':
			new_lst[-1] += lst[i+1]
			skip_next = True
		else:
			new_lst.append(lst[i])
	return new_lst

class Evaluator:
	def __init__(self, expression_lst):
		self.expressions = []
		if not isinstance(expression_lst, list):
			raise TypeError
		
		for e in expression_lst:
			if isinstance(e, list):
				self.expressions.append(Evaluator(e))
			elif isinstance(e, str):
				self.expressions.append(e)
			else:
				raise TypeError

		# atomic = len(self.expressions) == 1

		self.handle_subtract()

		self.break_operations()

		if self.operation_at_begin_or_end():
			raise ValueError

		if self.consecutive_operation():
			raise ValueError

		self.make_numbers()

		# if atomic:
		# 	print(Evaluator.str(self.expressions))
		# 	print(self.evaluate())
		# 	print()
	def str(exp):
		ret = ""
		for e in exp:
			if Evaluator.is_operation(e):
				ret += str(e) + " "
			else:
				ret += "(" + str(e) + ") "
		return ret


	def evaluate(self):
		new_expressions = []
		for e in self.expressions:
			if isinstance(e, Evaluator):
				new_expressions.append(e.evaluate())
			else:
				new_expressions.append(e)

		new_expressions = compute_mult(new_expressions)
		new_expressions = compute_add(new_expressions)
		return new_expressions[0]

	def handle_subtract(self):
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
		new_expressions = []
		for e in self.expressions:
			if not isinstance(e, str):
				new_expressions.append(e)
				continue
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
	evaluator_input = break_parenthesis(remove_all_spaces(string))
	return Evaluator(evaluator_input).evaluate()

def start_interface():
	print("Starting interface. Enter q to leave")
	while True:
		expression = input(">> ")
		if expression == 'q':
			print('exit')
			break
		elif '=' in expression:
			print(expression)
			parts = expression.split('=')
			if len(parts) != 2:
				print("Error, try again.")

			name = remove_all_spaces(parts[0])
			if not name.isalpha():
				print("Wrong name")
				continue

			values[name] = evaluate_complex_number(parts[1])
			print(values)
		else:
			try:
				print(evaluate_complex_number(expression))
			except:
				print("Error, try again.")

if __name__ == "__main__":

	start_interface()



