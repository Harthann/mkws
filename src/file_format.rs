pub const CLASS_CPP: &[u8] = b"#include \"__name__.hpp\"

__name__::__name__()
{

}

__name__::__name__(__name__ const &x)
{

}

__name__::__name__	&__name__::operator=(__name__ const &x)
{
	return (*this);
}

__name__::~__name__()
{

}
";

pub const CLASS_HPP: &[u8] = b"#ifndef __NAME___HPP
#define __NAME___HPP

class __name__
{
	public:
		__name__();
		__name__(__name__ const &);
		__name__ &operator=(__name__ const&);
		~__name__();
	protected:
	private:
};

#endif
";

pub const HEADER_HPP: &[u8] = b"#ifndef __NAME___HPP
#define __NAME___HPP

#endif
";

pub const INTERFACE_HPP: &[u8] = b"#ifndef __NAME___HPP
#define __NAME___HPP

class __name__
{
	public:
		__name__();
		virtual ~__name__();
	protected:
	private:
};

#endif
";
