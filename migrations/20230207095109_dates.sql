ALTER TABLE article
    ADD COLUMN created TIMESTAMP NOT NULL DEFAULT now();