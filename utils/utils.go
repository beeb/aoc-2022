package utils

import (
	"errors"
	"io"
	"net/http"
	"os"
	"strconv"

	"github.com/AlecAivazis/survey/v2"
	"github.com/AlecAivazis/survey/v2/terminal"
)

func getDayInput(day int) ([]byte, error) {
	session, err := os.ReadFile("../../session.txt")
	if err != nil {
		return nil, err
	}
	req, err := http.NewRequest("GET", "https://adventofcode.com/2022/day/"+strconv.Itoa(day)+"/input", nil)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Cookie", "session="+string(session))
	client := http.Client{}
	res, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()
	resBody, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, err
	}
	return resBody, nil
}

func WriteDayInput(day int) error {
	if _, err := os.Stat("input.txt"); os.IsNotExist(err) {
		// check if session.txt file exists
		if _, err := os.Stat("../../session.txt"); os.IsNotExist(err) {
			session := ""
			promptSession := &survey.Input{
				Message: "Enter your session key",
			}
			err := survey.AskOne(promptSession, &session)
			if err == terminal.InterruptErr {
				return errors.New("user cancelled")
			}
			// store session key in session.txt
			err = os.WriteFile("../../session.txt", []byte(session), 0644)
			if err != nil {
				return err
			}
		}

		input, err := getDayInput(1)
		if err != nil {
			return err
		}
		f, err := os.Create("input.txt")
		if err != nil {
			return err
		}
		defer f.Close()
		_, err = f.Write(input)
		if err != nil {
			return err
		}
	}
	return nil
}
