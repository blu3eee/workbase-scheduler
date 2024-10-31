```sql
CREATE TABLE IF NOT EXISTS users (
    id BIGINT NOT NULL PRIMARY KEY,
    email VARCHAR(100) NOT NULL UNIQUE,
    encrypted_password VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    date_of_birth DATE NOT NULL,
    phone_number VARCHAR(20) NULL UNIQUE,
    avatar VARCHAR(255) NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    deleted_at TIMESTAMP NULL,
);

CREATE TABLE IF NOT EXISTS companies (
    id BIGINT NOT NULL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    icon VARCHAR(255) NULL,
    last_employee_id INT NOT NULL DEFAULT 0,
    owner_id BIGINT NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS company_employees (
    id INT NOT NULL,
    user_id BIGINT NOT NULL,
    company_id BIGINT NOT NULL,
    hired_date DATE NOT NULL DEFAULT (CURRENT_DATE),
    punch_id INT NOT NULL,
    notes TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    UNIQUE KEY company_emp (user_id, company_id)
);

CREATE TABLE IF NOT EXISTS company_locations (
    id BIGINT NOT NULL PRIMARY KEY,
    company_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    timezone VARCHAR(50) NOT NULL DEFAULT 'America/Los_Angeles',
    address TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
);

CREATE TABLE IF NOT EXISTS company_onboarding_invites (
    id BIGINT NOT NULL PRIMARY KEY,
    company_id BIGINT NOT NULL,
    location_id BIGINT NOT NULL,
    email VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    role_id BIGINT,
    status ENUM('PENDING', 'CANCELLED', 'APPROVED', 'DENIED') NOT NULL DEFAULT 'PENDING',
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES department_roles(id) ON DELETE SET NULL,
);

CREATE TABLE IF NOT EXISTS location_departments (
    id BIGINT NOT NULL PRIMARY KEY,
    location_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS department_roles (
    id BIGINT NOT NULL PRIMARY KEY,
    department_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    wage FLOAT NOT NULL,
    color VARCHAR(6) NOT NULL DEFAULT FFFFFF,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (department_id) REFERENCES location_departments(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS location_holiday_settings (
    id BIGINT NOT NULL PRIMARY KEY,
    location_id BIGINT NOT NULL,
    holiday DATE NOT NULL,
    is_closed BOOLEAN NOT NULL DEFAULT TRUE,
    factor FLOAT,
    open_time TIME,
    close_time TIME,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE,
    UNIQUE KEY location_holiday_setting (location_id, holiday)
);

CREATE TABLE IF NOT EXISTS location_operation_hours (
    id BIGINT NOT NULL PRIMARY KEY,
    location_id BIGINT NOT NULL,
    day_of_week ENUM('MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY', 'SUNDAY') NOT NULL,
    is_closed BOOLEAN NOT NULL DEFAULT TRUE,
    open_time TIME,
    close_time TIME,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE,
    UNIQUE KEY daily_operation_hour (location_id, day_of_week)
);

CREATE TABLE IF NOT EXISTS location_shift_feedback_settings (
    location_id BIGINT NOT NULL PRIMARY KEY,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS department_role_stations (
    role_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    FOREIGN KEY (role_id) REFERENCES department_roles(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift_feedback_settings (
    location_id BIGINT NOT NULL,
    enabled BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS location_operation_hours (
    location_id BIGINT NOT NULL,
    day_of_week ENUM('MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY', 'SUNDAY') NOT NULL,
    is_closed BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE,
    UNIQUE location_operation_day (location_id, day_of_week)
);

CREATE TABLE IF NOT EXISTS location_holiday_settings (
    location_id BIGINT NOT NULL,
    holiday DATE NOT NULL,
    factor FLOAT NOT NULL,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS availability_requests (
    id BIGINT NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    location_id BIGINT NOT NULL,
    start_date DATE NOT NULL,
    status ENUM('PENDING', 'CANCELLED', 'APPROVED', 'DENIED') NOT NULL DEFAULT 'PENDING',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS availability_details (
    request_id BIGINT NOT NULL,
    day_of_week ENUM('MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY', 'SUNDAY') NOT NULL,
    is_available BOOLEAN NOT NULL,
    whole_day BOOLEAN,
    preferred_start_time TIME,
    preferred_end_time TIME,
    start_time TIME,
    enmd_time TIME,
    FOREIGN KEY (request_id) REFERENCES availability_requests(id) ON DELETE CASCADE,
    UNIQUE availability_day_of_week (request_id, day_of_week)
);

```
