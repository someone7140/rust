CREATE TABLE "person_in_charges" (
    "id" BIGINT NOT NULL,
    "name" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_person_in_charges_by_name" ON "person_in_charges" ("name");
