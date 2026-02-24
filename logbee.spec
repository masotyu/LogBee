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
# システムの /tmp にあるバイナリを直接 install するように指定
install -p -m 755 /tmp/logbee %{buildroot}/usr/bin/logbee

%files
/usr/bin/logbee

%changelog
* Tue Feb 24 2026 masotyu - 0.1.0
- Initial RPM release