create table entries 
(
  id serial primary key,
  name text not null,
  description text not null,
  slug text not null,
  created_at timestamp not null default CURRENT_TIMESTAMP,
  updated_at timestamp not null default CURRENT_TIMESTAMP
);

create unique index entries_slug_uindex on entries (slug);

create trigger set_updated_at_timestamp
after update on entries
begin
   update entries set updated_at = CURRENT_TIMESTAMP where id = new.id;
end;