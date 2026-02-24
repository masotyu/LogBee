# %{_version} が外部から定義されていない場合のデフォルト値
%{!?_version: %define _version 0.1.0}

Name:           logbee
Version:        %{_version}
Release:        1%{?dist}
Summary:        TUI log viewer powered by DuckDB

License:        MIT
URL:            https://github.com/masotyu/LogBee
# ソースはビルド済みのバイナリを使用する想定です

%description
LogBee is a TUI log viewer designed for JSON log files, powered by DuckDB.

%install
mkdir -p %{buildroot}/usr/bin
# %{_builddir} を付けて、確実にコピー元の場所を指定する
install -p -m 755 %{_builddir}/logbee %{buildroot}/usr/bin/logbee

%files
/usr/bin/logbee

%changelog
* Tue Feb 24 2026 masotyu - 0.1.0
- Initial RPM release