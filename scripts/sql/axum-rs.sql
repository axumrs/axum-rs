-- 标签
CREATE TABLE IF NOT EXISTS "tags" (
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL UNIQUE,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE
);

-- 专题状态
CREATE TYPE "subject_status" AS ENUM ('Writing', 'Finished');

-- 专题
CREATE TABLE  IF NOT EXISTS "subjects" (
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL,
    "slug" VARCHAR(100) NOT NULL,
    "summary" VARCHAR(255) NOT NULL,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    "cover" VARCHAR(100) NOT NULL DEFAULT '',
    "status" subject_status NOT NULL DEFAULT 'Writing',
    "price" DECIMAL(10,2) NOT NULL DEFAULT 0,
    "pin" INTEGER NOT NULL DEFAULT 0,
    UNIQUE(slug)
) ;

-- 文章
CREATE TABLE IF NOT EXISTS "topics" (
    "id" CHAR(20)  PRIMARY KEY ,
    "title" VARCHAR(255) NOT NULL,
    "subject_id" CHAR(20)  NOT NULL,
    "slug" VARCHAR(100) NOT NULL,
    "summary" VARCHAR(255) NOT NULL,
    "author" VARCHAR(50) NOT NULL,
    "src" VARCHAR(50) NOT NULL,
    "hit" BIGINT  NOT NULL DEFAULT 0,
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "try_readable" BOOLEAN NOT NULL DEFAULT FALSE,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    "cover" VARCHAR(100) NOT NULL DEFAULT '',
    "md" VARCHAR NOT NULL,
    "pin" INTEGER NOT NULL DEFAULT 0,
    UNIQUE("subject_id", "slug")
);

-- 文章段落
CREATE TABLE IF NOT EXISTS "topic_sections"(
    "id" CHAR(20)  PRIMARY KEY ,
    "topic_id" CHAR(20) NOT NULL,
    "sort" INTEGER NOT NULL,
    "hash"  CHAR(64) NOT NULL,
    "content" VARCHAR
);

-- 文章-标签
CREATE TABLE IF NOT EXISTS "topic_tags" (
    "id" CHAR(20)  PRIMARY KEY ,
    "topic_id" CHAR(20) NOT NULL,
    "tag_id" CHAR(20) NOT NULL,
    UNIQUE("topic_id","tag_id")
);

-- 管理员
CREATE TABLE IF NOT EXISTS "admins" (
    "id" CHAR(20) PRIMARY KEY ,
    "username" VARCHAR(50) NOT NULL,
    "password" VARCHAR(72) NOT NULL,
    UNIQUE("username")
);

-- 用户状态
CREATE TYPE "user_status" AS ENUM ('Pending', 'Actived', 'Freezed');
-- 用户类型
CREATE TYPE "user_kind" AS ENUM ('Normal', 'Subscriber', 'YearlySubscriber');

-- 用户
CREATE TABLE IF NOT EXISTS "users" (
    "id" CHAR(20) PRIMARY KEY,
    "email" VARCHAR(255) NOT NULL,
    "nickname" VARCHAR(30) NOT NULL,
    "password" VARCHAR(72) NOT NULL,
    "status" user_status DEFAULT 'Pending',
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "kind" user_kind  NOT NULL DEFAULT 'Normal',
    "sub_exp" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08',
    "points" DECIMAL(8,0)  NOT NULL DEFAULT 0,
    "allow_device_num" SMALLINT  NOT NULL DEFAULT 1,
    "session_exp" SMALLINT  NOT NULL DEFAULT 0,
    UNIQUE("email"),
    UNIQUE("nickname")
);

-- 激活码类型
CREATE TYPE "activation_kind" AS ENUM('Active', 'ResetPassword');

-- 激活码
CREATE UNLOGGED TABLE  IF NOT EXISTS "activation_codes"(
    "id" CHAR(20) PRIMARY KEY,
    "email" VARCHAR(255) NOT NULL,
    "code"  CHAR(20) NOT NULL UNIQUE,
    "kind" activation_kind NOT NULL DEFAULT 'Active',
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expire_time" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08'
);

-- 用户登录日志
CREATE UNLOGGED TABLE IF NOT EXISTS "login_logs"(
     "id" CHAR(20) PRIMARY KEY,
     "user_id" CHAR(20) NOT NULL,
     "ip" VARCHAR(39) NOT NULL DEFAULT '',
     "user_agent" VARCHAR NOT NULL DEFAULT '',
     "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 服务
CREATE TABLE IF NOT EXISTS "services" (
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL,
    -- 是否专题
    "is_subject" BOOLEAN NOT NULL DEFAULT FALSE,
    -- 目标ID
    "target_id" CHAR(20) NOT NULL,
    --时效(天)
    "duration" SMALLINT NOT NULL DEFAULT 0,
    -- 价格
    "price" DECIMAL(10,2) NOT NULL,
    -- 封面
    "cover" VARCHAR(100) NOT NULL DEFAULT '',
    -- 是否允许积分兑换
    "allow_pointer" BOOLEAN NOT NULL DEFAULT FALSE,
    -- 普通用户折扣
    "normal_discount" SMALLINT NOT NULL DEFAULT 0,
    -- 订阅用户折扣
    "sub_discount" SMALLINT NOT NULL DEFAULT 0,
    -- 年费用户折扣
    "yearly_sub_discount" SMALLINT NOT NULL DEFAULT 0,
    -- 是否下架
    "is_off" BOOLEAN NOT NULL DEFAULT FALSE,
    -- 说明
    "desc" VARCHAR NOT NULL DEFAULT '',
    -- 排序
    "pin" INTEGER NOT NULL DEFAULT 0
);

-- 订单状态
CREATE TYPE "order_status" AS ENUM ('Pending', 'Finished', 'Cancelled', 'Closed');

-- 订单
CREATE TABLE IF NOT EXISTS "orders"(
    "id" CHAR(20) PRIMARY KEY ,
    -- 用户
    "user_id" CHAR(20) NOT NULL,
    -- 金额
    "amount" DECIMAL(10,2) NOT NULL,
    -- 实付金额
    "actual_amount" DECIMAL(10,2) NOT NULL,
    -- 状态
    "status" order_status NOT NULL DEFAULT 'Pending',
     -- 快照（服务详情&数量&金额（折扣前后）&用户ID&用户类型）
    "snapshot" VARCHAR NOT NULL,
    -- 是否允许积分兑换
    "allow_pointer" BOOLEAN NOT NULL DEFAULT FALSE,
    -- 创建时间
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);



-- 签到日志
CREATE TABLE IF NOT EXISTS "check_in_logs"(
	"id" CHAR(20) PRIMARY KEY ,
	"user_id" CHAR(20) NOT NULL,
	"points"  SMALLINT NOT NULL DEFAULT 0,
	"dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE "pay_applies_status" AS ENUM ('Pending', 'Reject', 'Finished');

-- 支付申请
CREATE TABLE  IF NOT EXISTS "pay_applies" (
	"id" CHAR(20) PRIMARY KEY ,
	"order_id" CHAR(20) NOT NULL,
	"user_id" CHAR(20) NOT NULL,
	"amount" DECIMAL(10,2) NOT NULL,
	"currency" currency_kind NOT NULL DEFAULT 'USDT',
	"kind" pay_kind NOT NULL DEFAULT 'TronLink',
	"tx_id" VARCHAR(255) NOT NULL DEFAULT '',
	"status" pay_applies_status NOT NULL DEFAULT 'Pending',
	"dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"is_del" BOOLEAN NOT NULL DEFAULT FALSE,
	"img" VARCHAR(255) NOT NULL DEFAULT '',
	"process_dateline" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08',
	"reason" VARCHAR(255) NOT NULL DEFAULT ''
);

-- 阅读历史
CREATE TABLE  IF NOT EXISTS "read_histories" (
    "id" CHAR(20) PRIMARY KEY,
    "user_id" CHAR(20) NOT NULL,
    "subject_slug" VARCHAR(100) NOT NULL,
    "slug" VARCHAR(100) NOT NULL,
    "subject_name" VARCHAR(100) NOT NULL,
    "topic_title" VARCHAR(255) NOT NULL,
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNLOGGED TABLE  IF NOT EXISTS "sessions"(
    "id" CHAR(20) PRIMARY KEY,
    "user_id" CHAR(20) NOT NULL,
    "token"  CHAR(64) NOT NULL UNIQUE,
    "is_admin" BOOLEAN NOT NULL DEFAULT FALSE,
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "loc" VARCHAR(100) NOT NULL DEFAULT '',
    "ip" VARCHAR(39) NOT NULL DEFAULT '',
    "ua" VARCHAR NOT NULL DEFAULT '',
    "expire_time" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08'
);

CREATE UNLOGGED TABLE IF NOT EXISTS "protected_contents"(
    "id" CHAR(20)  PRIMARY KEY ,
    "section_id" CHAR(20) NOT NULL,
    "content" VARCHAR,
    "expire_time" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08'
);