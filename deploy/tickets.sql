-- auto-generated definition
create table tickets
(
    id        int auto_increment
        primary key,
    code_num  varchar(500)               not null,
    create_at datetime default curtime() not null,
    update_at datetime default curtime() not null
)
    collate = utf8mb4_bin;
