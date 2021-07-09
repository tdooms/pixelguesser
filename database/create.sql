create database pixelguesser;
\c pixelguesser;


drop table if exists quizzes cascade;
create table quizzes
(
    quiz_id      bigserial primary key,

    name         text        not null,
    creator      text        not null,
    description  text        not null,
    image_url    text        not null,
    time_created timestamptz not null
);

drop table if exists rounds cascade;
create table rounds
(
    round_id  bigserial not null,
    quiz_id   bigint    not null,

    image_url text      not null,
    points    bigint    not null,
    guesses   bigint    not null,
    answer    text      not null,
    speed     float,


    foreign key (quiz_id) references quizzes (quiz_id),
    primary key (round_id, quiz_id)
);

insert into quizzes
values (default, 'Cities', 'Thomas Dooms', 'Popular monuments in popular cities', 'city.jpg', NOW());
insert into quizzes
values (default, 'Animals', 'Thomas Dooms', 'Well-known animals', 'animal.jpg', NOW());
insert into quizzes
values (default, 'Celebrities', 'Thomas Dooms', 'Famous people around the world', 'celebrity.jpg', NOW());
insert into quizzes
values (default, 'Christmas', 'Thomas Dooms', 'Anything and everything that involves christmas', 'christmas.jpg',
        NOW());
insert into quizzes
values (default, 'Logos', 'Thomas Dooms', 'Like logo quiz but better', 'logo.jpg', NOW());
insert into quizzes
values (default, 'Foods', 'Thomas Dooms', 'Tasty! But can you guess it quickly?', 'food.jpg', NOW());

insert into rounds
values (default, 1, 'paris.jpg', 1, 1, 'Paris');
insert into rounds
values (default, 1, 'pisa.jpg', 2, 1, 'Pisa');
insert into rounds
values (default, 1, 'london.jpg', 2, 1, 'London');
insert into rounds
values (default, 1, 'brussels.jpg', 2, 1, 'Brussels');
insert into rounds
values (default, 1, 'washington.jpg', 2, 1, 'Washington');
insert into rounds
values (default, 1, 'moscow.jpg', 3, 1, 'Moscow');
insert into rounds
values (default, 1, 'sydney.jpg', 3, 1, 'Sydney');
insert into rounds
values (default, 1, 'rio.jpg', 2, 1, 'Rio');
insert into rounds
values (default, 1, 'los_angeles.jpg', 3, 1, 'Los Angeles');
insert into rounds
values (default, 1, 'rome.jpg', 2, 1, 'Rome');
insert into rounds
values (default, 1, 'new_york_liberty.jpg', 3, 1, 'New York');
insert into rounds
values (default, 1, 'giza.jpg', 2, 1, 'Giza or Cairo');
insert into rounds
values (default, 1, 'las_vegas.jpg', 4, 1, 'Las Vegas');
insert into rounds
values (default, 1, 'berlin.jpg', 3, 1, 'Berlin');
insert into rounds
values (default, 1, 'agra.jpg', 3, 1, 'Paris');
insert into rounds
values (default, 1, 'barcelona.jpg', 4, 1, 'Barcelona');
insert into rounds
values (default, 1, 'new_york_times.jpg', 5, 1, 'New York');

insert into rounds
values (default, 2, 'elephant.jpg', 1, 1, 'Elephant');
insert into rounds
values (default, 2, 'panda.jpg', 1, 1, 'Panda');
insert into rounds
values (default, 2, 'dog.jpg', 1, 1, 'Dog');
insert into rounds
values (default, 2, 'penguin.jpg', 2, 1, 'Penguin');
insert into rounds
values (default, 2, 'giraffe.jpg', 2, 1, 'Giraffe');
insert into rounds
values (default, 2, 'bunny.jpg', 2, 1, 'Bunny');
insert into rounds
values (default, 2, 'lion.jpg', 3, 1, 'Lion');
insert into rounds
values (default, 2, 'rhino.jpg', 2, 1, 'Rhino');
insert into rounds
values (default, 2, 'pig.jpg', 2, 1, 'Pig');
insert into rounds
values (default, 2, 'fox.jpg', 2, 1, 'Fox');
insert into rounds
values (default, 2, 'elk.jpg', 2, 1, 'Elk');
insert into rounds
values (default, 2, 'cat.jpg', 2, 1, 'Cat');
insert into rounds
values (default, 2, 'tiger.jpg', 3, 1, 'Tiger');
insert into rounds
values (default, 2, 'snake.jpg', 3, 1, 'Snake');
insert into rounds
values (default, 2, 'gorilla.jpg', 3, 1, 'Gorilla');
insert into rounds
values (default, 2, 'owl.jpg', 3, 1, 'Owl');
insert into rounds
values (default, 2, 'peacock.jpg', 4, 1, 'Peacock');

insert into rounds
values (default, 3, 'obama.jpg', 1, 1, 'Barack Obama');
insert into rounds
values (default, 3, 'michel.jpg', 2, 1, 'Charles Michel');
insert into rounds
values (default, 3, 'trump.jpg', 1, 1, 'Donald Trump');
insert into rounds
values (default, 3, 'elizabeth.jpg', 2, 1, 'Queen Elizabeth');
insert into rounds
values (default, 3, 'smith.jpg', 2, 1, 'Will Smith');
insert into rounds
values (default, 3, 'zuckerberg.jpg', 2, 1, 'Marc Zuckerberg');
insert into rounds
values (default, 3, 'putin.jpg', 2, 1, 'Vladimir Putin');
insert into rounds
values (default, 3, 'mercury.jpg', 2, 1, 'Freddie Mercury');
insert into rounds
values (default, 3, 'jong_un.jpg', 2, 1, 'Kim Jong Un');
insert into rounds
values (default, 3, 'musk.jpg', 3, 1, 'Elon Musk');
insert into rounds
values (default, 3, 'hitler.jpg', 3, 1, 'Adolf Hitler');

insert into rounds
values (default, 4, 'tree.jpg', 1, 1, 'Christmas tree');
insert into rounds
values (default, 4, 'ball.jpg', 1, 1, 'Christmas ball');
insert into rounds
values (default, 4, 'mistletoe.jpg', 2, 1, 'Mistletoe');
insert into rounds
values (default, 4, 'rudolph.jpg', 2, 1, 'Rudolph');
insert into rounds
values (default, 4, 'present.jpg', 1, 1, 'Present');
insert into rounds
values (default, 4, 'snowman.jpg', 2, 1, 'Snowman');
insert into rounds
values (default, 4, 'buche.jpg', 2, 1, 'Buche');
insert into rounds
values (default, 4, 'crib.jpg', 2, 1, 'Crib');
insert into rounds
values (default, 4, 'gluhwein.jpg', 3, 1, 'Gluhwein');
insert into rounds
values (default, 4, 'santa.jpg', 2, 1, 'Santa Claus');

insert into rounds
values (default, 6, 'apple.jpg', 1, 1, 'Apple');
insert into rounds
values (default, 6, 'hamburger.jpg', 1, 1, 'Hamburger');
insert into rounds
values (default, 6, 'pizza.jpg', 2, 1, 'Pizza');
insert into rounds
values (default, 6, 'banana.jpg', 2, 1, 'Banana');
insert into rounds
values (default, 6, 'fries.jpg', 1, 1, 'Fries');
insert into rounds
values (default, 6, 'salad.jpg', 3, 1, 'Salad');
insert into rounds
values (default, 6, 'spaghetti.jpg', 2, 1, 'Spaghetti');
insert into rounds
values (default, 6, 'steak.jpg', 2, 1, 'Steak');
insert into rounds
values (default, 6, 'pancakes.jpg', 2, 1, 'Pancakes');
insert into rounds
values (default, 6, 'ice_cream.jpg', 2, 1, 'Ice Cream');
insert into rounds
values (default, 6, 'english.jpg', 4, 1, 'English breakfast');
