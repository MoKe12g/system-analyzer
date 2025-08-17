PRAGMA foreign_keys = ON;
create table dpkg_packages
(
    package_name TEXT
        constraint dpkg_packages_pk primary key not null,
    version        TEXT not null,
    date_installed datetime not null
);

create table files
(
    path       TEXT
        constraint files_pk primary key not null,
    size       integer                  not null,
    is_folder  boolean                  not null,
    package    TEXT,
    is_changed boolean,                                                              -- Can be null because file entry gets added before dpkg gets queried
    FOREIGN KEY (package) REFERENCES dpkg_packages (package_name) on delete set null -- https://sqlite.org/foreignkeys.html
);
