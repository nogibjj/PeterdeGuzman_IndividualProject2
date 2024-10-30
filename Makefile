install:
	pip install --upgrade pip && pip install -r requirements.txt

format:
	black *.py

lint:
	ruff check *.py mylib/*.py test_*.py 

test: 
	python -m pytest -vv --nbval -cov=mylib -cov=main test_main.py 

all: install format lint test 