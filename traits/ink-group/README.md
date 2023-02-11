
# Ink-Group Spec

Specification for a basic implementation of a group membership. It can be combinaned with other implementation for example a multisigs.
The main purpose of this specification is to create a group of members managed by an admin to be used as a basis in other implementations.

## Storage Struct

`Member { addr, weight }`

We define the struct that rappresent a member of the group where `addr(AccountId)` is the public address and `weight(u64)` is the voting power of that member.

## Messages

We define the messages that a group must expose:

### Get Admin

```http
  get_admin() -> Return the actual admin 
```

### Get Members

```http
  get_members() -> Return the list of the actual members
```

### Get Member

```http
  get_member(member) -> Return the Member info 
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `member`      | `AccountId` | **Required**. Public address of the member to search |

### Get Total Voting Power

```http
  get_total_weight() ->  Return the total voting power weight of the group
```

### Update Admin

```http
  update_admin(admin) -> Update the admin 
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `admin`      | `AccountId` | **Required**. Public address of the new admin |

### Update Members

```http
  update_members(new_members,remove_members) -> Update the members of the group
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `new_members`      | `Vec<Member>` | **Required**. vector of member/members |
| `remove_members`      | `Vec<Member>` | **Required**. vector of member/members |

## Errors

Enum of errors that the messages may response with:

`enum InkGroupError`

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
|       | `LogicErr` | Logic contract error (error that should not fire) |
|       | `Unauthorized` | Unauthorized |
| `member:AccountId`      | `DuplicateMember` | Entered duplicate member |
|       | `ZeroMembers` | No member entered |
|       | `NoMember` | Member not found |

