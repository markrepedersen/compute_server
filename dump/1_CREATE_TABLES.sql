CREATE TABLE USERS (
       id int NOT NULL AUTO_INCREMENT,
       first_name varchar(255) NOT NULL,
       last_name varchar(255) NOT NULL,
       email varchar(255) NOT NULL,
       username varchar(255) NOT NULL,
       password varchar(255) NOT NULL,
       salt varchar(255) NOT NULL,
       PRIMARY KEY (id)
);

CREATE TABLE DATASET (
       id int NOT NULL AUTO_INCREMENT,
       user_id int NOT NULL,
       dataset_path varchar(255) NOT NULL,
       config_path varchar(255) NOT NULL,
       FOREIGN KEY (user_id)
       	       REFERENCES USERS(id)
	       ON DELETE CASCADE
);
