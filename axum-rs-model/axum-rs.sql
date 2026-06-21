-- 专题状态
CREATE TYPE "subject_status" AS ENUM ('Writing', 'Finished');

-- 专题
CREATE TABLE  IF NOT EXISTS "subjects" (
    "id" UUID DEFAULT uuidv7() PRIMARY KEY,
    "name" TEXT NOT NULL,
    "slug" TEXT NOT NULL,
    "summary" TEXT NOT NULL,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    "cover" TEXT NOT NULL DEFAULT '',
    "status" subject_status NOT NULL DEFAULT 'Writing',
    "price" INTEGER CHECK("price" >= 0) NOT NULL DEFAULT 0,
    "pin" INTEGER NOT NULL DEFAULT 0,
    UNIQUE(slug)
) ;

-- 文章
CREATE TABLE IF NOT EXISTS "topics" (
    "id" UUID DEFAULT uuidv7() PRIMARY KEY,
    "title" TEXT NOT NULL,
    "subject_id" UUID  NOT NULL,
    "slug" TEXT NOT NULL,
    "summary" TEXT NOT NULL,
    "author" TEXT NOT NULL,
    "src" TEXT NOT NULL,
    "hit" BIGINT CHECK("hit" >= 0)  NOT NULL DEFAULT 0,
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "try_readable" BOOLEAN NOT NULL DEFAULT FALSE,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    "cover" TEXT NOT NULL DEFAULT '',
    "md" TEXT NOT NULL,
    "sections" TEXT[] NOT NULL DEFAULT '{}',
    "tags" TEXT[] NOT NULL DEFAULT '{}',
    "pin" INTEGER NOT NULL DEFAULT 0,
    UNIQUE("subject_id", "slug")
);

-- 管理员
CREATE TABLE IF NOT EXISTS "admins" (
    "id" UUID DEFAULT uuidv7() PRIMARY KEY ,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    UNIQUE("username")
);

-- 用户状态
CREATE TYPE "user_status" AS ENUM ('Pending', 'Actived', 'Freezed');
-- 用户类型
CREATE TYPE "user_kind" AS ENUM ('Normal', 'Subscriber', 'YearlySubscriber');
-- 用户来源
CREATE TYPE "user_source" AS ENUM ('Email', 'Github', 'Google', 'Telegram');

-- 用户
CREATE TABLE IF NOT EXISTS "users" (
    "id" UUID DEFAULT uuidv7() PRIMARY KEY,
    "identifier" TEXT NOT NULL, -- 邮箱、Telegram ID
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "status" user_status DEFAULT 'Pending',
    "source" user_source NOT NULL DEFAULT 'Email',
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "kind" user_kind  NOT NULL DEFAULT 'Normal',
    "sub_exp" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 00:00:00+00:00',
    "points" BIGINT CHECK("points" >= 0)  NOT NULL DEFAULT 0,
    "allow_device_num" SMALLINT  NOT NULL DEFAULT 1,
    "session_exp" SMALLINT  NOT NULL DEFAULT 0,
    UNIQUE("identifier"),
    UNIQUE("username"),
);