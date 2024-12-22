Name:           Rhyolite
Version:        0.1.0
Release:        1%{?dist}
Summary:        Rhyolite - A simple text editor written in Rust using Tauri, inspired by Obsidian.

License:        Apache-2.0
URL:            https://www.apache.org/licenses/LICENSE-2.0
Source0:        %{name}-%{version}.tar.gz
Source1:        %{name}-%{version}.tar.gz/%{name}.png
Source2:        %{name}-%{version}.tar.gz/%{name}.desktop

Requires:       glibc, gtk3, webkit2gtk3

%description
Rhyolite - A simple text editor written in Rust using Tauri, inspired by Obsidian.


%prep
%autosetup


%build
%global debug_package %{nil}
CFLAGS="%{optflags}"


%install
# Install the binary
install -Dm755 %{_builddir}/%{name}-%{version}/%{name} %{buildroot}%{_bindir}/%{name}

# Install the icon
install -Dm644 %{_builddir}/%{name}-%{version}/%{name}.png %{buildroot}%{_datadir}/icons/hicolor/1080x1080/apps/%{name}.png

# Install the .desktop file
install -Dm644 %{_builddir}/%{name}-%{version}/%{name}.desktop %{buildroot}%{_datadir}/applications/%{name}.desktop


%files
%{_bindir}/%{name}
%{_datadir}/icons/hicolor/1080x1080/apps/%{name}.png
%{_datadir}/applications/%{name}.desktop



%changelog
* Tue Dec 17 2024 Suyog Tandel <127536688+RedddFoxxyy@users.noreply.github.com>
-
