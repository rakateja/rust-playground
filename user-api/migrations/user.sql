CREATE DATABASE `rust_user_project`;
USE `rust_user_project`;

CREATE TABLE `user` (
    `id` INT AUTO_INCREMENT,
    `username` VARCHAR(50) CHARACTER SET utf8 NOT NULL UNIQUE,
    `email` VARCHAR(50) CHARACTER SET utf8 NOT NULL UNIQUE,
    `password` VARCHAR(100) CHARACTER SET utf8 NOT NULL,
    `display_name` VARCHAR(100) CHARACTER SET utf8 NOT NULL,
    `avatar` VARCHAR(100) CHARACTER SET utf8 NOT NULL DEFAULT "",
    `active` BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;