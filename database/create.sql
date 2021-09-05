create table aggregates (
	id bigint primary key not null, -- should probably be guid
	aggregate_name varchar(50) not null,
	version int not null
);

create table event_log (
	id bigint primary key not null auto_increment,
	aggregate_id bigint not null,
	version int not null,
	event_type int not null,
	json_data json not null,
		
	constraint aggregates_fk foreign key (aggregate_id) references aggregates(id)
);