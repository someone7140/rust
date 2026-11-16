CREATE TABLE "agencies" (
    "id" BIGINT NOT NULL,
    "name" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_agencies_by_name" ON "agencies" ("name");
CREATE TABLE "laws" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "law_number" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "departments" (
    "id" BIGINT NOT NULL,
    "name" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_departments_by_name" ON "departments" ("name");
CREATE TABLE "budget_summaries" (
    "project_id" BIGINT NOT NULL,
    "budget_year" INTEGER NOT NULL,
    "initial_budget" BIGINT NOT NULL,
    "supplementary_budget" BIGINT NOT NULL,
    "carried_over_budget" BIGINT NOT NULL,
    "contingency_budget" BIGINT NOT NULL,
    "summary_budget" BIGINT NOT NULL,
    "executuon_budget" BIGINT NOT NULL,
    "executuon_rate" DOUBLE PRECISION,
    "carried_forward_budget" BIGINT NOT NULL,
    "request_budget" BIGINT NOT NULL,
    "reason_for_change" TEXT,
    PRIMARY KEY ("project_id", "budget_year")
);
CREATE TABLE "sections" (
    "id" BIGINT NOT NULL,
    "name" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_sections_by_name" ON "sections" ("name");
CREATE TABLE "relation_projects" (
    "project_id" BIGINT NOT NULL,
    "relation_project_id" BIGINT NOT NULL,
    "relationship" TEXT NOT NULL,
    PRIMARY KEY ("project_id", "relation_project_id")
);
CREATE TABLE "offices" (
    "id" BIGINT NOT NULL,
    "name" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_offices_by_name" ON "offices" ("name");
CREATE TABLE "ministries" (
    "id" BIGINT NOT NULL,
    "name" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_ministries_by_name" ON "ministries" ("name");
CREATE TABLE "projects" (
    "project_id" BIGINT NOT NULL,
    "project_name" TEXT NOT NULL,
    "responsible_ministry_id" BIGINT NOT NULL,
    "ministry_id" BIGINT NOT NULL,
    "agency_id" BIGINT,
    "department_id" BIGINT,
    "section_name" BIGINT,
    "office_name" BIGINT,
    "person_in_charge_id" BIGINT,
    "project_purpose" TEXT NOT NULL,
    "project_situation" TEXT NOT NULL,
    "project_overview" TEXT NOT NULL,
    "overview_url" TEXT,
    "start_year" BIGINT,
    "end_year" BIGINT,
    "policy" TEXT,
    "policy_measure" TEXT,
    "policy_url" TEXT,
    "law_id" TEXT,
    "law_article" TEXT,
    "law_paragraph" TEXT,
    "law_item" TEXT,
    "plan_notification_name" TEXT,
    "plan_notification_url" TEXT,
    "subsidy_target" TEXT,
    "subsidy_rate" TEXT,
    "subsidy_limit" TEXT,
    "subsidy_url" TEXT,
    PRIMARY KEY ("project_id")
);
CREATE TABLE "person_in_charges" (
    "id" BIGINT NOT NULL,
    "name" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_person_in_charges_by_name" ON "person_in_charges" ("name");
