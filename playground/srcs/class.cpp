#include "class.hpp"

test::test()
: x(6)
{
}

const int &test::getX() const
{
	return (x);
}