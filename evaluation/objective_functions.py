import numpy as np

def rosenbrock(x, derivatives, parameters = {}):
	"""
	Evaluate the Rosenbrock function 

		f(x) = (a-x)^2 + b(y-x^2)^2

	and its derivatives.

	Accepts:
		                 x: point of evaluation (list or numpy array with ndim == 1)
		       derivatives: Boolean list of length 3 indicating which derivatives
		                    (orders 0-2) should be returned; e.g., [True, True, False]
		        parameters: optional parameters (dictionary);
		                    the following key/value pairs are evaluated:
		             ["a"]: parameter 1
		             ["b"]: parameter 2

	Returns:
		       values: dictionary containing the evaluations of the function
		               and its first and second derivatives as required, with
		               keys ["function"], ["derivative"] and ["Hessian"]
	"""
  
	a = parameters.get("a", 1)
	b = parameters.get("b", 100)
  
	values = {}
	if derivatives[0]:
		values["function"] = (a-x[0])**2 + b*(x[1]-x[0]**2)**2
	if derivatives[1]:
		values["derivative"] = np.array([\
			-2 * (a-x[0]) - 4 * b * x[0] * (x[1]-x[0]**2),\
			2 * b * (x[1]-x[0]**2)\
		])
	if derivatives[2]:
		values["Hessian"] = np.array([\
			[2 - 4 * b * (x[1] - 3 * x[0]**2), -4 * b * x[0]],\
			[-4 * b * x[0], 2 * b]\
		])
    
	return values
