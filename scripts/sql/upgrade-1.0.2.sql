ALTER TABLE user_read_history ENGINE=INNODB;

ALTER TABLE subject ADD COLUMN pin TINYINT UNSIGNED NOT NULL DEFAULT '0';
ALTER TABLE subject ADD INDEX IDX_SUBJECT_PIN(pin);

ALTER TABLE topic ADD COLUMN pin TINYINT UNSIGNED NOT NULL DEFAULT '0';
ALTER TABLE topic ADD INDEX IDX_TOPIC_PIN(pin);

ALTER  VIEW v_topic_web_list AS
SELECT 
	t.id, t.title, t.slug, t.try_readable, t.cover,t.summary ,t.hit,t.pin
	, s.name AS subject_name , s.slug  as subject_slug, s.pin as subject_pin
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

ALTER VIEW v_topic_admin_list AS
SELECT 
	t.id, t.title, t.slug, t.hit, t.dateline, t.try_readable, t.is_del, t.cover,t.pin
	, s.name AS subject_name , s.slug  as subject_slug 
FROM 
	topic AS t 
INNER JOIN
	subject as s 
ON 
	t.subject_id = s.id 
;