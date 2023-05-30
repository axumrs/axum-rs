CREATE TABLE subject (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    summary VARCHAR(255) NOT NULL,
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    cover VARCHAR(100) NOT NULL DEFAULT '',
    status TINYINT UNSIGNED NOT NULL DEFAULT 0,
    price INT UNSIGNED NOT NULL DEFAULT 0,
    UNIQUE(slug)
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_subject_slug ON subject (slug);

CREATE TABLE topic (
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    subject_id INT UNSIGNED NOT NULL REFERENCES subject(id),
    slug VARCHAR(100) NOT NULL,
    summary VARCHAR(255) NOT NULL,
    author VARCHAR(50) NOT NULL,
    src VARCHAR(50) NOT NULL,
    hit BIGINT UNSIGNED NOT NULL DEFAULT 0,
    dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    try_readable BOOLEAN NOT NULL DEFAULT FALSE,
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    cover VARCHAR(100) NOT NULL DEFAULT '',
    UNIQUE(subject_id, slug)
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_topic_slug ON topic (slug);

CREATE TABLE topic_content (
    topic_id BIGINT UNSIGNED NOT NULL PRIMARY KEY REFERENCES topic(id),
    md TEXT NOT NULL,
    html TEXT NOT NULL
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE tag (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(name)
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_tag_name ON tag (name);

CREATE TABLE topic_tag (
    topic_id BIGINT UNSIGNED NOT NULL REFERENCES topic(id),
    tag_id INT UNSIGNED NOT NULL REFERENCES tag(id),
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(topic_id,tag_id)
)ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE admin (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(255) NOT NULL,
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(username)
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `user` (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    email VARCHAR(255) NOT NULL,
    nickname VARCHAR(30) NOT NULL,
    password VARCHAR(255) NOT NULL,
    status TINYINT UNSIGNED NOT NULL DEFAULT 0,
    dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    types TINYINT UNSIGNED NOT NULL DEFAULT 0,
    sub_exp DATETIME NOT NULL DEFAULT '1970-1-1 0:0:0',
    points INT UNSIGNED NOT NULL DEFAULT 0,
    allow_device_num TINYINT UNSIGNED NOT NULL DEFAULT 1,
    jwt_exp TINYINT UNSIGNED NOT NULL DEFAULT 0,
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(email),
    UNIQUE(nickname)
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `user_login_log` (
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    user_id INT UNSIGNED NOT NULL,
    ip VARCHAR(45) NOT NULL DEFAULT '',
    browser VARCHAR(50) NOT NULL DEFAULT  '',
    os VARCHAR(50) NOT NULL DEFAULT '',
    device VARCHAR(50) NOT NULL DEFAULT '',
    dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_del BOOLEAN NOT NULL DEFAULT FALSE
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `user_login_log_agent` (
    log_id BIGINT UNSIGNED PRIMARY KEY ,
    user_agent TINYTEXT NOT NULL
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `order` (
	id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
	user_id INT UNSIGNED NOT NULL,
	price INT UNSIGNED NOT NULL,
	status TINYINT UNSIGNED NOT NULL DEFAULT 0,
	code CHAR(7) NOT NULL DEFAULT '',
	full_code CHAR(64) NOT NULL DEFAULT '',
	order_num CHAR(20) NOT NULL DEFAULT '',
	dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	pay_id BIGINT UNSIGNED NOT NULL DEFAULT 0,
	is_del TINYINT UNSIGNED NOT NULL DEFAULT 0,
	UNIQUE (order_num)
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `order_snap` (
	order_id BIGINT UNSIGNED PRIMARY KEY,
	snap TEXT NOT NULL
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `pay` (
	id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
	order_id BIGINT UNSIGNED NOT NULL,
	user_id INT UNSIGNED NOT NULL,
	price INT UNSIGNED NOT NULL,
	currency TINYINT UNSIGNED NOT NULL DEFAULT 0,
	types TINYINT UNSIGNED NOT NULL DEFAULT 0,
	tx_id VARCHAR(255) NOT NULL DEFAULT '',
	status TINYINT UNSIGNED NOT NULL DEFAULT 0,
	dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	is_del TINYINT UNSIGNED NOT NULL DEFAULT 0
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE user_purchased_service(
	id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
	order_id BIGINT UNSIGNED NOT NULL,
	user_id INT UNSIGNED NOT NULL,
	service_id INT UNSIGNED NOT NULL,
	service_type TINYINT UNSIGNED NOT NULL DEFAULT 0,
	server_num INT UNSIGNED NOT NULL,
	status TINYINT UNSIGNED NOT NULL DEFAULT 0,
	dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE user_check_in(
	id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
	user_id INT UNSIGNED NOT NULL,
	points  INT UNSIGNED NOT NULL DEFAULT 0,
	dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `pay_apply` (
	id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
	order_id BIGINT UNSIGNED NOT NULL,
	user_id INT UNSIGNED NOT NULL,
	price INT UNSIGNED NOT NULL,
	currency TINYINT UNSIGNED NOT NULL DEFAULT 0,
	types TINYINT UNSIGNED NOT NULL DEFAULT 0,
	tx_id VARCHAR(255) NOT NULL DEFAULT '',
	status TINYINT UNSIGNED NOT NULL DEFAULT 0,
	dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	is_del TINYINT UNSIGNED NOT NULL DEFAULT 0,
	img VARCHAR(255) NOT NULL DEFAULT '',
	process_dateline DATETIME NOT NULL,
	reason VARCHAR(255) NOT NULL DEFAULT ''
) ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `user_read_history` (
    id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    user_id INT UNSIGNED NOT NULL,
    subject_slug VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    dateline DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_del BOOLEAN NOT NULL DEFAULT FALSE
) 
ENGINE=Archive CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;
-- ENGINE=INNODB CHARSET=UTF8MB4 COLLATE=utf8mb4_unicode_ci;

-- 初始数据

INSERT INTO `admin` (username, password) VALUES ('axum.rs', '$2b$12$NEBR/1uK0Hz82Ec2kXdsUuxyFLFZfe3cqs2blTuYSVwaOsHJvMS8e');
INSERT INTO `user` (email,nickname,password,status,types,sub_exp,points,allow_device_num,jwt_exp) VALUES ('team@axum.rs', 'root', '$2b$12$NmTubg.C3UMdWURqX54aDeP6xp0WEfcMHYtMdIT84cEMdDEJunYfq', 1, 1, '2999-12-31 23:59:59', 999999, 3, 120);

-- 视图

CREATE VIEW v_topic_admin_list AS
SELECT 
	t.id, t.title, t.slug, t.hit, t.dateline, t.try_readable, t.is_del, t.cover
	, s.name AS subject_name , s.slug  as subject_slug 
FROM 
	topic AS t 
INNER JOIN
	subject as s 
ON 
	t.subject_id = s.id 
;

CREATE  VIEW v_topic_web_list AS
SELECT 
	t.id, t.title, t.slug, t.try_readable, t.cover,t.summary ,t.hit 
	, s.name AS subject_name , s.slug  as subject_slug 
	, CONCAT(',',GROUP_CONCAT( tg.name),',')  as tag_names
FROM 
	topic AS t 
INNER JOIN
	subject as s 
ON 
	t.subject_id = s.id 
LEFT JOIN 
	topic_tag as tt 
ON 
	t.id=tt.topic_id 
INNER JOIN 
	tag as tg 
ON 
	tt.tag_id = tg.id
WHERE t.is_del = false
GROUP BY 
	t.id
;

CREATE  VIEW v_tag_web_list AS
SELECT 
	COUNT(t.id) AS topic_total,tg.name,tg.id
FROM 
	topic as t 
INNER JOIN 
	topic_tag as tt 
ON 
	t.id=tt.topic_id 
INNER JOIN 
	tag as tg 
ON tt.tag_id  = tg.id 
WHERE tg.is_del = false AND t.is_del =false
group by tg.id;


CREATE  VIEW v_topic_web_detail AS
SELECT 
	t.id, t.title, t.slug, t.try_readable, t.cover ,t.hit,t.dateline 
	, tc.html
	, s.name AS subject_name , s.slug  as subject_slug,s.price,s.id as subject_id
	, GROUP_CONCAT( tg.name)  as tag_names
FROM 
	topic AS t 
INNER JOIN
	subject as s 
ON 
	t.subject_id = s.id 
INNER JOIN 
	topic_content as tc 
ON 
	t.id  = tc.topic_id 
LEFT JOIN 
	topic_tag as tt 
ON 
	t.id=tt.topic_id 
INNER JOIN 
	tag as tg 
ON 
	tt.tag_id = tg.id
WHERE t.is_del = false
GROUP BY 
	t.id
;

CREATE VIEW v_user_login_log_full as
SELECT id, user_id, ip, browser, os, device, dateline, is_del,lla.user_agent
FROM user_login_log as ll
INNER JOIN user_login_log_agent as lla
on ll.id = lla.log_id;

CREATE VIEW v_order_full AS 
SELECT 
	id, user_id, price, status, code, full_code, order_num, dateline, pay_id, is_del,snap 
FROM 
	`order` as o 
INNER JOIN 
	order_snap as os 
ON o.id  = os.order_id ;

CREATE VIEW v_order_with_user AS 
SELECT 
	o.id, user_id, price, o.status, code, full_code, order_num, o.dateline, pay_id, o.is_del
	, u.email , u.nickname 
FROM 
	`order` as o 
INNER JOIN 
	`user` as u
ON o.user_id  = u.id ;

CREATE VIEW v_order_full_with_user AS 
SELECT 
	o.id, o.user_id, o.price, o.status, o.code, o.full_code, o.order_num, o.dateline, o.pay_id, o.is_del,snap 
	,ou.email,ou.nickname
FROM 
	v_order_with_user as ou
INNER JOIN 
	v_order_full as o
ON ou.id=o.id;


CREATE VIEW v_user_read_history AS
SELECT 
	urh.id,urh.dateline,urh.is_del
	, vtwl.id as topic_id , title, vtwl.slug, try_readable, cover, summary, hit, subject_name, vtwl.subject_slug, tag_names
	, u.id as user_id, email ,nickname 
FROM 
	user_read_history as urh 
INNER JOIN 
	v_topic_web_list as vtwl 
ON 
	urh.subject_slug = vtwl.subject_slug AND urh.slug = vtwl.slug 
INNER JOIN 
	`user` as u 
ON urh.user_id = u.id ;

CREATE VIEW v_user_purchased_subject AS
SELECT 
	 ups.id as purchased_id, order_id, user_id, service_id, service_type, server_num, ups.status as purchased_status, ups.dateline as purchased_dateline
	 ,u.email,u.nickname
	 ,s.id,s.slug,s.name,s.summary,s.cover,s.status,s.price,s.is_del

FROM 
	user_purchased_service as ups 
INNER JOIN 
	`user` as u 
ON 
	ups.user_id = u.id
INNER JOIN
	subject as s 
ON 
	ups.service_id=s.id 
WHERE 
	ups.service_type=1 AND ups.status=1;

CREATE VIEW v_user_purchased_service as
SELECT 
	 ups.id , order_id, ups.user_id, service_id, service_type, server_num, ups.status , ups.dateline
	 ,u.email,u.nickname
	  ,s.id as subject_id,s.slug as subject_slug,s.name as subject_name,s.summary as subject_summary,s.cover as subject_cover,s.status as subject_status,s.price as subject_price,s.is_del as subject_is_del
	  ,o.order_num

FROM 
	user_purchased_service as ups 
left join
	`order` as o 
on ups.order_id=o.id
left JOIN 
	`user` as u 
ON 
	ups.user_id = u.id
LEFT JOIN 
	subject as s 
ON 
	ups.service_id=s.id AND ups.service_type=1
	;


