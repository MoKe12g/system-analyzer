create table dpkg_packages
(
    package_name   VARCHAR(50)
        constraint dpkg_packages_pk
            unique,
    version        VARCHAR(50)
        constraint dpkg_packages_pk_2
            unique,
    date_installed datetime
);

create table files
(
    path       VARCHAR(50)
        constraint files_pk
            primary key,
    size       integer,
    is_folder  boolean,
    package    VARCHAR(50),
    is_changed boolean
);
