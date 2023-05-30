-- 初始数据

-- 初始管理员（必须）。用户名和密码均为 axum.rs
INSERT INTO `admin` (username, password) VALUES ('axum.rs', '$2b$12$NEBR/1uK0Hz82Ec2kXdsUuxyFLFZfe3cqs2blTuYSVwaOsHJvMS8e');

-- 初始用户（非必须）。邮箱为 team@axum.rs 密码为 axum.rs
INSERT INTO `user` (email,nickname,password,status,types,sub_exp,points,allow_device_num,jwt_exp) VALUES ('team@axum.rs', 'root', '$2b$12$NmTubg.C3UMdWURqX54aDeP6xp0WEfcMHYtMdIT84cEMdDEJunYfq', 1, 1, '2999-12-31T23:59:59+08:00', 999999, 3, 120);