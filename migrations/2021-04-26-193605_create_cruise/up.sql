-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `stock_history` (
    `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
    `ticker` varchar(4) NOT NULL,
    `date` date NOT NULL,
    `high` float NOT NULL,
    `low` float NOT NULL,
    `open` float NOT NULL,
    `close` float NOT NULL,
    `volume` float NOT NULL,
    `adj_close` float NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `cruise_data` (
    `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
    `brand` varchar(300) NOT NULL,
    `total_passengers` float NOT NULL,
    `passengers_percentage` float NOT NULL,
    `revenue` float NOT NULL,
    `revenue_percentage` float NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;