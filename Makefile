clean:
	rm -rf ./build
	rm -rf ./lib
	rm -rf ./dist
	`which cargo` clean

release_linux:
	docker pull quay.io/pypa/manylinux2014_x86_64
	docker run --rm -v `pwd`:/io quay.io/pypa/manylinux2014_x86_64 /io/build_wheels.sh

release_macosx:
	python setup.py bdist_wheel

release: clean release_linux release_macosx
	twine upload -r nexus --config-file ./.pypirc dist/*

