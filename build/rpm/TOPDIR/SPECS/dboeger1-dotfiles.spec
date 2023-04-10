Name: dboeger1-dotfiles
Version: 0.1
Release: 1%{?dist}
Summary: Collection of GitHub user dboeger1's personal configuration files.

License:
URL:
Source0:

BuildRequires:
Requires:

%description


%prep
%autosetup


%build
%configure
%make_build


%install
%make_install


%check


%files
%license
%doc


%changelog
