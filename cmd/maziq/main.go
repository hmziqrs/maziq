package main

import (
	"fmt"
	"os"
	"strings"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

// Styles
var (
	// Colors
	primaryColor   = lipgloss.Color("#00D9FF")
	secondaryColor = lipgloss.Color("#7C3AED")
	accentColor    = lipgloss.Color("#10B981")
	mutedColor     = lipgloss.Color("#6B7280")
	errorColor     = lipgloss.Color("#EF4444")

	// Title style
	titleStyle = lipgloss.NewStyle().
			Foreground(primaryColor).
			Bold(true).
			Padding(0, 1).
			MarginTop(1).
			MarginBottom(1)

	// Logo ASCII art style
	logoStyle = lipgloss.NewStyle().
			Foreground(secondaryColor).
			Bold(true)

	// Subtitle style
	subtitleStyle = lipgloss.NewStyle().
			Foreground(mutedColor).
			Italic(true).
			MarginBottom(1)

	// Box style for content sections
	boxStyle = lipgloss.NewStyle().
			Border(lipgloss.RoundedBorder()).
			BorderForeground(primaryColor).
			Padding(1, 2).
			MarginTop(1).
			MarginBottom(1)

	// Menu item styles
	menuItemStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#E5E7EB")).
			PaddingLeft(2)

	selectedMenuItemStyle = lipgloss.NewStyle().
				Foreground(primaryColor).
				Bold(true).
				PaddingLeft(0)

	// Help style
	helpStyle = lipgloss.NewStyle().
			Foreground(mutedColor).
			Padding(1, 0)

	// Status indicator styles
	readyStyle = lipgloss.NewStyle().
			Foreground(accentColor).
			Bold(true)

	errorStyle = lipgloss.NewStyle().
			Foreground(errorColor).
			Bold(true)
)

type model struct {
	width         int
	height        int
	selectedMenu  int
	menuItems     []string
	ready         bool
}

func initialModel() model {
	return model{
		menuItems: []string{
			"Software Catalog",
			"Templates",
			"E2E Testing",
			"Configuration",
		},
		ready: true,
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		return m, nil

	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit

		case "up", "k":
			if m.selectedMenu > 0 {
				m.selectedMenu--
			}

		case "down", "j":
			if m.selectedMenu < len(m.menuItems)-1 {
				m.selectedMenu++
			}

		case "enter", " ":
			// TODO: Handle menu selection
		}
	}
	return m, nil
}

func (m model) View() string {
	if m.width == 0 {
		return "Loading..."
	}

	var sections []string

	// Logo and title
	logo := logoStyle.Render(`
 ███╗   ███╗ █████╗ ███████╗██╗ ██████╗
 ████╗ ████║██╔══██╗╚══███╔╝██║██╔═══██╗
 ██╔████╔██║███████║  ███╔╝ ██║██║   ██║
 ██║╚██╔╝██║██╔══██║ ███╔╝  ██║██║▄▄ ██║
 ██║ ╚═╝ ██║██║  ██║███████╗██║╚██████╔╝
 ╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚═╝ ╚══▀▀═╝ `)

	subtitle := subtitleStyle.Render("macOS Provisioning & Automation Tool")

	header := lipgloss.JoinVertical(lipgloss.Center, logo, subtitle)
	sections = append(sections, header)

	// Status indicator
	var status string
	if m.ready {
		status = readyStyle.Render("● Ready")
	} else {
		status = errorStyle.Render("● Not Ready")
	}
	statusBox := boxStyle.Width(m.width - 4).Render(status)
	sections = append(sections, statusBox)

	// Menu
	var menuItems []string
	for i, item := range m.menuItems {
		var renderedItem string
		if i == m.selectedMenu {
			renderedItem = selectedMenuItemStyle.Render("❯ " + item)
		} else {
			renderedItem = menuItemStyle.Render("  " + item)
		}
		menuItems = append(menuItems, renderedItem)
	}

	menu := strings.Join(menuItems, "\n")
	menuBox := boxStyle.
		Width(m.width - 4).
		Render(menu)
	sections = append(sections, menuBox)

	// Help text
	help := helpStyle.Render(
		"↑/↓ or j/k: Navigate • Enter: Select • q: Quit",
	)
	sections = append(sections, help)

	// Join all sections
	content := lipgloss.JoinVertical(lipgloss.Left, sections...)

	// Center the content
	return lipgloss.Place(
		m.width,
		m.height,
		lipgloss.Center,
		lipgloss.Center,
		content,
	)
}

func main() {
	p := tea.NewProgram(
		initialModel(),
		tea.WithAltScreen(),
		tea.WithMouseCellMotion(),
	)

	if _, err := p.Run(); err != nil {
		fmt.Printf("Error: %v", err)
		os.Exit(1)
	}
}
