#include "main.hpp"
#include "class.hpp"
#include "min.hpp"

int main(void)
{
	test A;
	int a = 5;
	int b = 2;
	std::cout << "Hello World" << std::endl;
	std::cout << A.getX() << std::endl;
	std::cout << min(a, b) << std::endl;
	return (0);
}