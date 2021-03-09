#ifndef MIN_T_HPP
#define MIN_T_HPP

template <class T>
T	min(T& a, T&b)
{
	if (a < b)
		return a;
	return b;
}


#endif