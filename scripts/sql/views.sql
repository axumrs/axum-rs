-- 文章-专题关联视图
CREATE VIEW "v_topic_subjects" AS 
SELECT 
    t.id, t.title, t.subject_id, t.slug, t.summary, t.author, t.src, t.hit, t.dateline, t.try_readable, t.is_del, t.cover, t.md, t.pin
    ,s."name", s.slug AS subject_slug, s.summary AS subject_summary, s.is_del AS subject_is_del, s.cover AS subject_cover, s.status, s.price, s.pin AS subject_pin
FROM
    topics AS t
INNER JOIN
    subjects AS s
ON
    t.subject_id = s.id
;

-- 文章标签-标签关联视图
CREATE VIEW "v_topic_tag_with_tags" AS
SELECT 
    t.id, t."name", t.is_del
    , tt.id AS topic_tag_id, topic_id
FROM 
    topic_tags AS tt
INNER JOIN
    tags AS t
ON 
  tt.tag_id = t.id
;

-- 订单-用户关联视图
CREATE VIEW "v_order_users" AS
SELECT 
	o.id, user_id, amount, actual_amount, o.status, "snapshot", allow_pointer, o.dateline
	, u.email, u.nickname 
FROM 
orders AS o
INNER JOIN
users AS u
ON o.user_id = u.id
;