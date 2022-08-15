# Drop Tables to Re-Create Database
DROP TABLE IF EXISTS
`tblMovies`,
`tblMovieRatings`;

# Movie Ratings (G, PG, PG-13, etc)
CREATE TABLE `tblMovieRatings` (
	`ratingID` int NOT NULL AUTO_INCREMENT,
    `rating` varchar(6),
    PRIMARY KEY (`ratingID`)
);

CREATE TABLE `tblMovies` (
  `movieID` int NOT NULL AUTO_INCREMENT,
  `movieName` varchar(2000) DEFAULT NULL,
  `movieReleaseYear` year DEFAULT NULL,
  `movieRating` int DEFAULT NULL,
  `movieDesc` varchar(4000) DEFAULT NULL,
  `movieStreamLocation` varchar(4000) DEFAULT NULL,
  `movieCoverLocation` varchar(4000) DEFAULT NULL,
  PRIMARY KEY (`movieID`),
  CONSTRAINT `FK_MovieRating_Movies` FOREIGN KEY (`movieRating`) REFERENCES `tblMovieRatings` (`ratingID`)
);

# Create/Re-create user for application access
DROP USER IF EXISTS 'beamScotty';
CREATE USER 'beamScotty'@'%' IDENTIFIED BY 'B3amM3UpScotty^^';

# Grant Access to the scotty database
GRANT INSERT, UPDATE, DELETE, SELECT, EXECUTE ON scotty.* TO 'beamScotty'@'%';