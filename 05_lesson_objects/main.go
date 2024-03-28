package main

import "fmt"

// Object oriented programming
// Everything is an object

type Gender string

const (
	GenderMale   Gender = "male"
	GenderFemale Gender = "female"
)

func NewCharacter(userName string, gender Gender) Character {
	return Character{
		UserName:    userName,
		Gender:      gender,
		CombatLevel: 1,
		CharacterEquipment: Equipment{
			Weapon: Weapon{
				Type:         WeaponTypeSword,
				MaterialType: MaterialTypeIron,
			},
		},
	}
}

type Character struct {
	UserName           string
	Gender             Gender
	CombatLevel        int
	CharacterEquipment Equipment
}

type Equipment struct {
	Weapon Weapon
	Amulet Amulet
	Cape   Cape
}

type Amulet struct {
	AttackBonus  int
	DefenceBonus int
}

type Cape struct {
	AttackBonus int
}

type WeaponType string

const (
	WeaponTypeMace  WeaponType = "mace"
	WeaponTypeSword WeaponType = "sword"
)

type MaterialType string

const (
	MaterialTypeIron MaterialType = "Iron"
	MaterialTypeRune MaterialType = "Rune"
)

type Weapon struct {
	Name         string
	Type         WeaponType
	MaterialType MaterialType
	Speed        int
	Attack       int
	Bludgening   int
	Peircing     int
	Crushing     int
}

func (w Weapon) DisplayName() string {
	return fmt.Sprintf("%v %v", w.MaterialType, w.Type)
}

func main() {
	// Logan
	var myCharacter Character = NewCharacter("loghen41", GenderMale)

	fmt.Println(myCharacter)

	fmt.Println(myCharacter.CharacterEquipment.Weapon.DisplayName())

	myCharacter.CharacterEquipment.Weapon.MaterialType = MaterialTypeRune

	fmt.Println(myCharacter.CharacterEquipment.Weapon.DisplayName())

	var RyansCharacter Character = NewCharacter("luckySandwic", GenderMale)
	fmt.Println(RyansCharacter)
	fmt.Println(RyansCharacter.CharacterEquipment.Weapon.DisplayName())

	fmt.Println(RyansCharacter == myCharacter)

}
